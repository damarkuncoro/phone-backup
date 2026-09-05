use crate::domain::VideoMetadata;

/// Extracts technical metadata from MP4/MOV/3GP byte streams.
pub struct Mp4Extractor;

impl Mp4Extractor {
    /// Inspects MP4 header bytes to extract resolution, duration, and codec.
    pub fn extract_from_bytes(data: &[u8]) -> Option<VideoMetadata> {
        let mut offset = 0;
        let mut width = 0u32;
        let mut height = 0u32;
        let mut duration_secs = 0.0f64;
        let mut codec: Option<String> = None;

        while offset + 8 <= data.len() {
            let size = u32::from_be_bytes([data[offset], data[offset + 1], data[offset + 2], data[offset + 3]]) as usize;
            let tag = &data[offset + 4..offset + 8];

            let atom_size = if size == 1 {
                if offset + 16 > data.len() { break; }
                u64::from_be_bytes(data[offset + 8..offset + 16].try_into().ok()?) as usize
            } else if size == 0 {
                data.len() - offset
            } else {
                size
            };

            if atom_size < 8 || offset + atom_size > data.len() {
                // If container has moov atom inside, search within data
                if tag == b"moov" {
                    let moov_slice = &data[offset + 8..];
                    Self::parse_moov(moov_slice, &mut width, &mut height, &mut duration_secs, &mut codec);
                }
                break;
            }

            if tag == b"moov" {
                let moov_slice = &data[offset + 8..offset + atom_size];
                Self::parse_moov(moov_slice, &mut width, &mut height, &mut duration_secs, &mut codec);
                break;
            }

            offset += atom_size;
        }

        if width > 0 || height > 0 || duration_secs > 0.0 {
            let mut meta = VideoMetadata::new(width, height, duration_secs);
            meta.video_codec = codec;
            Some(meta)
        } else {
            None
        }
    }

    fn parse_moov(
        data: &[u8],
        width: &mut u32,
        height: &mut u32,
        duration: &mut f64,
        codec: &mut Option<String>,
    ) {
        // Search for mvhd
        if let Some(mvhd_idx) = find_subsequence(data, b"mvhd") {
            let start = mvhd_idx + 4;
            if start + 24 <= data.len() {
                let version = data[start];
                if version == 0 && start + 20 <= data.len() {
                    let timescale = u32::from_be_bytes(data[start + 12..start + 16].try_into().unwrap_or_default()) as f64;
                    let dur = u32::from_be_bytes(data[start + 16..start + 20].try_into().unwrap_or_default()) as f64;
                    if timescale > 0.0 {
                        *duration = dur / timescale;
                    }
                } else if version == 1 && start + 28 <= data.len() {
                    let timescale = u32::from_be_bytes(data[start + 20..start + 24].try_into().unwrap_or_default()) as f64;
                    let dur = u64::from_be_bytes(data[start + 24..start + 32].try_into().unwrap_or_default()) as f64;
                    if timescale > 0.0 {
                        *duration = dur / timescale;
                    }
                }
            }
        }

        // Search for tkhd (track header for video track resolution)
        let mut search_idx = 0;
        while let Some(tkhd_rel) = find_subsequence(&data[search_idx..], b"tkhd") {
            let tkhd_idx = search_idx + tkhd_rel + 4;
            if tkhd_idx + 84 <= data.len() {
                let w = u16::from_be_bytes([data[tkhd_idx + 76], data[tkhd_idx + 77]]) as u32;
                let h = u16::from_be_bytes([data[tkhd_idx + 80], data[tkhd_idx + 81]]) as u32;
                if w > *width || h > *height {
                    *width = w;
                    *height = h;
                }
            }
            search_idx = tkhd_idx + 4;
        }

        // Search for video codec tags (avc1 for H.264, hvc1/hev1 for H.265, vp09, av01)
        if find_subsequence(data, b"avc1").is_some() {
            *codec = Some("H.264 / AVC".to_string());
        } else if find_subsequence(data, b"hvc1").is_some() || find_subsequence(data, b"hev1").is_some() {
            *codec = Some("H.265 / HEVC".to_string());
        } else if find_subsequence(data, b"vp09").is_some() {
            *codec = Some("VP9".to_string());
        } else if find_subsequence(data, b"av01").is_some() {
            *codec = Some("AV1".to_string());
        } else if find_subsequence(data, b"mp4v").is_some() {
            *codec = Some("MPEG-4".to_string());
        }
    }
}

fn find_subsequence(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack.windows(needle.len()).position(|window| window == needle)
}

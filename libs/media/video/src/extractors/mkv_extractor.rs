use crate::domain::VideoMetadata;

/// Extracts technical metadata from Matroska (MKV) and WebM byte streams.
pub struct MkvExtractor;

impl MkvExtractor {
    /// Inspects MKV/WebM EBML headers to extract resolution, duration, and codec.
    pub fn extract_from_bytes(data: &[u8]) -> Option<VideoMetadata> {
        if data.len() < 12 || !data.starts_with(&[0x1A, 0x45, 0xDF, 0xA3]) {
            return None;
        }

        let mut width = 0u32;
        let mut height = 0u32;
        let mut duration_secs = 0.0f64;
        let mut codec: Option<String> = None;

        // Search for PixelWidth (0xB0) and PixelHeight (0xBA)
        if let Some(pos) = find_ebml_id(data, &[0xB0]) {
            if let Some(val) = read_ebml_uint(&data[pos + 1..]) {
                width = val as u32;
            }
        }

        if let Some(pos) = find_ebml_id(data, &[0xBA]) {
            if let Some(val) = read_ebml_uint(&data[pos + 1..]) {
                height = val as u32;
            }
        }

        // Search for CodecID (0x86)
        if let Some(pos) = find_ebml_id(data, &[0x86]) {
            if let Some(codec_str) = read_ebml_string(&data[pos + 1..]) {
                codec = Some(clean_codec_name(&codec_str));
            }
        }

        // Search for Duration (0x44, 0x89)
        if let Some(pos) = find_ebml_id(data, &[0x44, 0x89]) {
            if pos + 2 + 4 <= data.len() {
                let dur_bytes = &data[pos + 2..pos + 6];
                if let Ok(bytes) = dur_bytes.try_into() {
                    let raw_dur = f32::from_be_bytes(bytes) as f64;
                    duration_secs = (raw_dur / 1000.0).max(0.0);
                }
            }
        }

        if width > 0 || height > 0 || duration_secs > 0.0 {
            let mut meta = VideoMetadata::new(width, height, duration_secs);
            meta.video_codec = codec;
            Some(meta)
        } else {
            None
        }
    }
}

fn find_ebml_id(data: &[u8], id: &[u8]) -> Option<usize> {
    data.windows(id.len()).position(|w| w == id)
}

fn read_ebml_uint(data: &[u8]) -> Option<u64> {
    if data.is_empty() { return None; }
    let len_byte = data[0];
    let len = if len_byte & 0x80 != 0 {
        1
    } else if len_byte & 0x40 != 0 {
        2
    } else if len_byte & 0x20 != 0 {
        3
    } else if len_byte & 0x10 != 0 {
        4
    } else {
        return None;
    };

    if data.len() < 1 + len { return None; }
    let mut val = 0u64;
    for &b in &data[1..1 + len] {
        val = (val << 8) | (b as u64);
    }
    Some(val)
}

fn read_ebml_string(data: &[u8]) -> Option<String> {
    if data.is_empty() { return None; }
    let len = (data[0] & 0x7F) as usize;
    if len == 0 || data.len() < 1 + len { return None; }
    String::from_utf8(data[1..1 + len].to_vec()).ok()
}

fn clean_codec_name(raw: &str) -> String {
    if raw.contains("AVC") || raw.contains("H264") {
        "H.264 / AVC".to_string()
    } else if raw.contains("HEVC") || raw.contains("H265") {
        "H.265 / HEVC".to_string()
    } else if raw.contains("VP9") {
        "VP9".to_string()
    } else if raw.contains("AV1") {
        "AV1".to_string()
    } else {
        raw.to_string()
    }
}

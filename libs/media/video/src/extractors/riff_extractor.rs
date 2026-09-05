use crate::domain::VideoMetadata;

/// Extracts technical metadata from RIFF AVI byte streams.
pub struct RiffExtractor;

impl RiffExtractor {
    /// Inspects AVI RIFF headers to extract resolution, duration, and codec.
    pub fn extract_from_bytes(data: &[u8]) -> Option<VideoMetadata> {
        if data.len() < 12 || !data.starts_with(b"RIFF") || &data[8..12] != b"AVI " {
            return None;
        }

        let mut width = 0u32;
        let mut height = 0u32;
        let mut duration_secs = 0.0f64;
        let mut fps: Option<f32> = None;
        let mut codec: Option<String> = None;

        // Find avih header
        if let Some(pos) = find_chunk(data, b"avih") {
            let start = pos + 8;
            if start + 40 <= data.len() {
                let usec_per_frame = u32::from_le_bytes(data[start..start + 4].try_into().unwrap_or_default());
                let total_frames = u32::from_le_bytes(data[start + 16..start + 20].try_into().unwrap_or_default());
                let w = u32::from_le_bytes(data[start + 32..start + 36].try_into().unwrap_or_default());
                let h = u32::from_le_bytes(data[start + 36..start + 40].try_into().unwrap_or_default());

                width = w;
                height = h;

                if usec_per_frame > 0 {
                    fps = Some(1_000_000.0 / (usec_per_frame as f32));
                    duration_secs = (total_frames as f64 * usec_per_frame as f64) / 1_000_000.0;
                }
            }
        }

        // Find strh chunk for video stream FourCC
        if let Some(pos) = find_chunk(data, b"strh") {
            let start = pos + 8;
            if start + 12 <= data.len() && &data[start..start + 4] == b"vids" {
                let fourcc = &data[start + 4..start + 8];
                if let Ok(fourcc_str) = std::str::from_utf8(fourcc) {
                    codec = Some(fourcc_str.to_uppercase());
                }
            }
        }

        if width > 0 || height > 0 || duration_secs > 0.0 {
            let mut meta = VideoMetadata::new(width, height, duration_secs);
            meta.fps = fps;
            meta.video_codec = codec;
            Some(meta)
        } else {
            None
        }
    }
}

fn find_chunk(data: &[u8], chunk_id: &[u8; 4]) -> Option<usize> {
    data.windows(4).position(|w| w == chunk_id)
}

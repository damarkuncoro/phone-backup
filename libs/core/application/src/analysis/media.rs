use domain::MediaInfo;
use image_engine::ExifReader;
use std::io::Cursor;

pub struct MediaAnalyzer;

impl MediaAnalyzer {
    pub fn extract_info(data: &[u8], mime_type: &str) -> Option<MediaInfo> {
        if mime_type.starts_with("image/") {
            return Self::extract_image_info(data);
        } else if mime_type == "video/mp4" || mime_type == "video/quicktime" {
            return Self::extract_video_info(data);
        }
        None
    }

    fn extract_image_info(data: &[u8]) -> Option<MediaInfo> {
        let meta = ExifReader::read_from_bytes(data).ok()?;
        let (lat, lon) = match meta.gps {
            Some(ref gps) => (Some(gps.latitude), Some(gps.longitude)),
            None => (None, None),
        };

        let (width, height) = if let Ok(img) = image::load_from_memory(data) {
            (Some(img.width()), Some(img.height()))
        } else {
            (None, None)
        };

        Some(MediaInfo {
            camera_make: meta.camera.make,
            camera_model: meta.camera.model,
            width,
            height,
            taken_at: meta.date_taken,
            latitude: lat,
            longitude: lon,
            duration_ms: None,
        })
    }

    fn extract_video_info(data: &[u8]) -> Option<MediaInfo> {
        let size = data.len() as u64;
        let reader = Cursor::new(data);
        if let Ok(mp4) = mp4::Mp4Reader::read_header(reader, size) {
            let mut width = None;
            let mut height = None;

            for track in mp4.tracks().values() {
                if let Ok(track_type) = track.track_type() {
                    if track_type == mp4::TrackType::Video {
                        width = Some(track.width() as u32);
                        height = Some(track.height() as u32);
                        break;
                    }
                }
            }
            return Some(MediaInfo {
                duration_ms: Some(mp4.duration().as_millis() as u64),
                width,
                height,
                ..Default::default()
            });
        }
        None
    }
}

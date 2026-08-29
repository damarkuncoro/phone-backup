use domain::MediaInfo;
use chrono::{Utc, DateTime};
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
        let mut reader = Cursor::new(data);
        if let Ok(exif_data) = exif::Reader::new().read_from_container(&mut reader) {
            let mut info = MediaInfo::default();

            if let Some(field) = exif_data.get_field(exif::Tag::Make, exif::In::PRIMARY) {
                info.camera_make = Some(field.display_value().to_string());
            }
            if let Some(field) = exif_data.get_field(exif::Tag::Model, exif::In::PRIMARY) {
                info.camera_model = Some(field.display_value().to_string());
            }
            if let Some(field) = exif_data.get_field(exif::Tag::PixelXDimension, exif::In::PRIMARY) {
                info.width = field.value.get_uint(0);
            }
            if let Some(field) = exif_data.get_field(exif::Tag::PixelYDimension, exif::In::PRIMARY) {
                info.height = field.value.get_uint(0);
            }

            if let Some(field) = exif_data.get_field(exif::Tag::DateTimeOriginal, exif::In::PRIMARY) {
                let date_str = field.display_value().to_string();
                if let Ok(dt) = chrono::NaiveDateTime::parse_from_str(&date_str, "%Y:%m:%d %H:%M:%S") {
                    info.taken_at = Some(DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc));
                }
            }

            if let (Some(lat_field), Some(lat_ref), Some(lon_field), Some(lon_ref)) = (
                exif_data.get_field(exif::Tag::GPSLatitude, exif::In::PRIMARY),
                exif_data.get_field(exif::Tag::GPSLatitudeRef, exif::In::PRIMARY),
                exif_data.get_field(exif::Tag::GPSLongitude, exif::In::PRIMARY),
                exif_data.get_field(exif::Tag::GPSLongitudeRef, exif::In::PRIMARY),
            ) {
                info.latitude = Self::parse_gps_coordinate(lat_field, lat_ref.display_value().to_string().contains('S'));
                info.longitude = Self::parse_gps_coordinate(lon_field, lon_ref.display_value().to_string().contains('W'));
            }

            return Some(info);
        }
        None
    }

    fn extract_video_info(data: &[u8]) -> Option<MediaInfo> {
        let size = data.len() as u64;
        let reader = Cursor::new(data);
        if let Ok(mp4) = mp4::Mp4Reader::read_header(reader, size) {
            let mut info = MediaInfo::default();
            info.duration_ms = Some(mp4.duration().as_millis() as u64);

            for track in mp4.tracks().values() {
                if let Ok(track_type) = track.track_type() {
                    if track_type == mp4::TrackType::Video {
                        info.width = Some(track.width() as u32);
                        info.height = Some(track.height() as u32);
                        break;
                    }
                }
            }
            return Some(info);
        }
        None
    }

    fn parse_gps_coordinate(field: &exif::Field, is_negative: bool) -> Option<f64> {
        if let exif::Value::Rational(ref values) = field.value {
            if values.len() >= 3 {
                let d = values[0].to_f64();
                let m = values[1].to_f64();
                let s = values[2].to_f64();
                let mut coord = d + (m / 60.0) + (s / 3600.0);
                if is_negative { coord = -coord; }
                return Some(coord);
            }
        }
        None
    }
}

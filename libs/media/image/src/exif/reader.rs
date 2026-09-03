use crate::model::{CameraSettings, ExifMetadata, GpsCoordinates};
use anyhow::Result;
use chrono::{NaiveDateTime, TimeZone, Utc};
use exif::{Exif, In, Reader, Tag, Value};
use std::io::Cursor;

pub struct ExifReader;

impl ExifReader {
    pub fn read_from_bytes(bytes: &[u8]) -> Result<ExifMetadata> {
        let mut cursor = Cursor::new(bytes);
        let exif = match Reader::new().read_from_container(&mut cursor) {
            Ok(exif) => exif,
            Err(_) => return Ok(ExifMetadata::default()),
        };

        let camera = Self::extract_camera_settings(&exif);
        let gps = Self::extract_gps(&exif);
        let date_taken = Self::extract_date_taken(&exif);
        let orientation = Self::extract_orientation(&exif);
        let software = Self::get_tag_string(&exif, Tag::Software);

        Ok(ExifMetadata {
            camera,
            gps,
            date_taken,
            orientation,
            software,
        })
    }

    fn extract_camera_settings(exif: &Exif) -> CameraSettings {
        CameraSettings {
            make: Self::get_tag_string(exif, Tag::Make),
            model: Self::get_tag_string(exif, Tag::Model),
            lens_model: Self::get_tag_string(exif, Tag::LensModel),
            iso: Self::get_tag_u32(exif, Tag::PhotographicSensitivity),
            exposure_time: Self::get_tag_string(exif, Tag::ExposureTime),
            f_number: Self::get_tag_f32(exif, Tag::FNumber),
            focal_length: Self::get_tag_f32(exif, Tag::FocalLength),
        }
    }

    fn extract_gps(exif: &Exif) -> Option<GpsCoordinates> {
        let lat_field = exif.get_field(Tag::GPSLatitude, In::PRIMARY)?;
        let lat_ref = exif.get_field(Tag::GPSLatitudeRef, In::PRIMARY)?;
        let lon_field = exif.get_field(Tag::GPSLongitude, In::PRIMARY)?;
        let lon_ref = exif.get_field(Tag::GPSLongitudeRef, In::PRIMARY)?;

        let lat = Self::parse_dms(&lat_field.value)?;
        let lat_sign = if lat_ref.display_value().to_string().contains('S') { -1.0 } else { 1.0 };

        let lon = Self::parse_dms(&lon_field.value)?;
        let lon_sign = if lon_ref.display_value().to_string().contains('W') { -1.0 } else { 1.0 };

        let alt = exif.get_field(Tag::GPSAltitude, In::PRIMARY).and_then(|f| match &f.value {
            Value::Rational(v) if !v.is_empty() => Some(v[0].to_f64()),
            _ => None,
        });

        Some(GpsCoordinates::new(lat * lat_sign, lon * lon_sign, alt))
    }

    fn parse_dms(val: &Value) -> Option<f64> {
        if let Value::Rational(rats) = val {
            if rats.len() >= 3 {
                let deg = rats[0].to_f64();
                let min = rats[1].to_f64();
                let sec = rats[2].to_f64();
                return Some(deg + (min / 60.0) + (sec / 3600.0));
            }
        }
        None
    }

    fn extract_date_taken(exif: &Exif) -> Option<chrono::DateTime<Utc>> {
        let date_str = Self::get_tag_string(exif, Tag::DateTimeOriginal)
            .or_else(|| Self::get_tag_string(exif, Tag::DateTime))?;
        NaiveDateTime::parse_from_str(&date_str, "%Y:%m:%d %H:%M:%S")
            .ok()
            .map(|dt| Utc.from_utc_datetime(&dt))
    }

    fn extract_orientation(exif: &Exif) -> Option<u32> {
        exif.get_field(Tag::Orientation, In::PRIMARY)
            .and_then(|f| f.value.get_uint(0))
    }

    fn get_tag_string(exif: &Exif, tag: Tag) -> Option<String> {
        exif.get_field(tag, In::PRIMARY)
            .map(|f| f.display_value().to_string().trim_matches('"').to_string())
    }

    fn get_tag_u32(exif: &Exif, tag: Tag) -> Option<u32> {
        exif.get_field(tag, In::PRIMARY).and_then(|f| f.value.get_uint(0))
    }

    fn get_tag_f32(exif: &Exif, tag: Tag) -> Option<f32> {
        exif.get_field(tag, In::PRIMARY).and_then(|f| match &f.value {
            Value::Rational(v) if !v.is_empty() => Some(v[0].to_f64() as f32),
            _ => None,
        })
    }
}

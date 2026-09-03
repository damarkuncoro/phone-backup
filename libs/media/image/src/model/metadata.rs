use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GpsCoordinates {
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: Option<f64>,
}

impl GpsCoordinates {
    pub fn new(latitude: f64, longitude: f64, altitude: Option<f64>) -> Self {
        Self {
            latitude,
            longitude,
            altitude,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CameraSettings {
    pub make: Option<String>,
    pub model: Option<String>,
    pub lens_model: Option<String>,
    pub iso: Option<u32>,
    pub exposure_time: Option<String>,
    pub f_number: Option<f32>,
    pub focal_length: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ExifMetadata {
    pub camera: CameraSettings,
    pub gps: Option<GpsCoordinates>,
    pub date_taken: Option<DateTime<Utc>>,
    pub orientation: Option<u32>,
    pub software: Option<String>,
}

impl ExifMetadata {
    pub fn has_gps(&self) -> bool {
        self.gps.is_some()
    }

    pub fn has_camera_info(&self) -> bool {
        self.camera.make.is_some() || self.camera.model.is_some()
    }
}

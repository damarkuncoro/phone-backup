use crate::client::AdbClient;
use crate::parsers::media_parser::MediaParser;
use crate::scripts::AndroidScripts;
use anyhow::Result;
use domain::{DeviceId, FileEntry};

/// Sub-scanner dedicated to querying Android MediaStore image and video providers.
#[derive(Clone)]
pub struct MediaStoreScanner {
    client: AdbClient,
}

impl MediaStoreScanner {
    pub fn new(client: AdbClient) -> Self {
        Self { client }
    }

    pub fn scan(&self, device_id: &DeviceId) -> Result<Vec<FileEntry>> {
        let mut all_media = Vec::new();

        let image_script = AndroidScripts::query_mediastore("image");
        if let Ok(image_out) = self.client.shell(&device_id.0, &image_script) {
            all_media.extend(MediaParser::parse_mediastore(device_id, &image_out));
        }

        let video_script = AndroidScripts::query_mediastore("video");
        if let Ok(video_out) = self.client.shell(&device_id.0, &video_script) {
            all_media.extend(MediaParser::parse_mediastore(device_id, &video_out));
        }

        Ok(all_media)
    }
}

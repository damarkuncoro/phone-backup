use crate::client::AdbClient;
use crate::parsers::media_parser::MediaParser;
use crate::scripts::AndroidScripts;
use anyhow::Result;
use domain::{DeviceId, FileEntry};

/// Sub-scanner dedicated to querying Android MediaStore image, video, and audio providers.
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

        let categories = ["image", "video", "audio"];
        for category in categories {
            let script = AndroidScripts::query_mediastore(category);
            if let Ok(output) = self.client.shell(&device_id.0, &script) {
                all_media.extend(MediaParser::parse_mediastore(device_id, &output));
            }
        }

        Ok(all_media)
    }
}

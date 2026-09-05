use crate::discovery::WhatsAppPathScanner;
use crate::model::{MediaCategory, WhatsAppMediaItem};
use chrono::{NaiveDate, TimeZone, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WhatsAppMediaSummary {
    pub total_files: usize,
    pub total_bytes: u64,
    pub voice_notes_count: usize,
    pub images_count: usize,
    pub videos_count: usize,
    pub documents_count: usize,
    pub sent_files_count: usize,
}

pub struct WhatsAppMediaIndexer;

impl WhatsAppMediaIndexer {
    /// Parses a single media file path into a structured WhatsAppMediaItem.
    pub fn parse_media_item(rel_path: &str, size_bytes: u64) -> WhatsAppMediaItem {
        let category = WhatsAppPathScanner::categorize_path(rel_path);
        let is_sent = rel_path.contains("/Sent/") || rel_path.contains("\\Sent\\");
        let filename = rel_path.split(['/', '\\']).next_back().unwrap_or(rel_path).to_string();
        let date_created = Self::extract_date_from_filename(&filename);

        let mut item = WhatsAppMediaItem::new(rel_path, filename, category, is_sent, size_bytes);
        item.date_created = date_created;
        item
    }

    /// Computes summary statistics over indexed media items.
    pub fn summarize(items: &[WhatsAppMediaItem]) -> WhatsAppMediaSummary {
        let mut summary = WhatsAppMediaSummary::default();
        for item in items {
            summary.total_files += 1;
            summary.total_bytes += item.size_bytes;
            if item.is_sent {
                summary.sent_files_count += 1;
            }
            match item.category {
                MediaCategory::VoiceNotes => summary.voice_notes_count += 1,
                MediaCategory::Images => summary.images_count += 1,
                MediaCategory::Video => summary.videos_count += 1,
                MediaCategory::Documents => summary.documents_count += 1,
                _ => {}
            }
        }
        summary
    }

    fn extract_date_from_filename(filename: &str) -> Option<chrono::DateTime<Utc>> {
        // Formats: IMG-YYYYMMDD-WAXXXX, PTT-YYYYMMDD-WAXXXX, VID-YYYYMMDD-WAXXXX
        let parts: Vec<&str> = filename.split('-').collect();
        if parts.len() >= 2 && parts[1].len() == 8 {
            if let Ok(naive_date) = NaiveDate::parse_from_str(parts[1], "%Y%m%d") {
                if let Some(dt) = naive_date.and_hms_opt(12, 0, 0) {
                    return Some(Utc.from_utc_datetime(&dt));
                }
            }
        }
        None
    }
}

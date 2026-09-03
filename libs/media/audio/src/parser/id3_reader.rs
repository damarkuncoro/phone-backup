use crate::model::AudioMetadata;

pub struct Id3Reader;

impl Id3Reader {
    /// Extracts ID3 metadata from MP3 byte stream.
    pub fn read_tags(bytes: &[u8]) -> AudioMetadata {
        let mut meta = AudioMetadata::new();

        // 1. Check ID3v2 at start of file
        if bytes.len() >= 10 && bytes.starts_with(b"ID3") {
            Self::parse_id3v2(bytes, &mut meta);
        }

        // 2. Fallback to ID3v1 at end of file (128 bytes)
        if bytes.len() >= 128 {
            let tail = &bytes[bytes.len() - 128..];
            if tail.starts_with(b"TAG") {
                Self::parse_id3v1(tail, &mut meta);
            }
        }

        meta
    }

    fn parse_id3v1(tail: &[u8], meta: &mut AudioMetadata) {
        if meta.title.is_none() {
            let title = String::from_utf8_lossy(&tail[3..33]).trim_matches('\0').trim().to_string();
            if !title.is_empty() {
                meta.title = Some(title);
            }
        }
        if meta.artist.is_none() {
            let artist = String::from_utf8_lossy(&tail[33..63]).trim_matches('\0').trim().to_string();
            if !artist.is_empty() {
                meta.artist = Some(artist);
            }
        }
        if meta.album.is_none() {
            let album = String::from_utf8_lossy(&tail[63..93]).trim_matches('\0').trim().to_string();
            if !album.is_empty() {
                meta.album = Some(album);
            }
        }
        if meta.year.is_none() {
            let year_str = String::from_utf8_lossy(&tail[93..97]).trim_matches('\0').trim().to_string();
            if let Ok(y) = year_str.parse::<u32>() {
                meta.year = Some(y);
            }
        }
    }

    fn parse_id3v2(bytes: &[u8], meta: &mut AudioMetadata) {
        let tag_size = ((bytes[6] as usize & 0x7F) << 21)
            | ((bytes[7] as usize & 0x7F) << 14)
            | ((bytes[8] as usize & 0x7F) << 7)
            | (bytes[9] as usize & 0x7F);

        let max_len = std::cmp::min(10 + tag_size, bytes.len());
        let mut offset = 10;

        while offset + 10 <= max_len {
            let frame_id = &bytes[offset..offset + 4];
            let frame_size = u32::from_be_bytes([
                bytes[offset + 4],
                bytes[offset + 5],
                bytes[offset + 6],
                bytes[offset + 7],
            ]) as usize;

            offset += 10;
            if offset + frame_size > max_len || frame_size == 0 {
                break;
            }

            let frame_data = &bytes[offset..offset + frame_size];
            if frame_data.len() > 1 {
                let content = String::from_utf8_lossy(&frame_data[1..]).trim_matches('\0').trim().to_string();
                if frame_id == b"TIT2" && meta.title.is_none() {
                    meta.title = Some(content);
                } else if frame_id == b"TPE1" && meta.artist.is_none() {
                    meta.artist = Some(content);
                } else if frame_id == b"TALB" && meta.album.is_none() {
                    meta.album = Some(content);
                }
            }

            offset += frame_size;
        }
    }
}

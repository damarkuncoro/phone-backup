use crate::model::AudioMetadata;

pub struct OggVorbisReader;

impl OggVorbisReader {
    /// Extracts Vorbis comments / Opus tags from Ogg bitstream.
    pub fn read_tags(bytes: &[u8]) -> AudioMetadata {
        let mut meta = AudioMetadata::new();

        if bytes.len() < 28 || !bytes.starts_with(b"OggS") {
            return meta;
        }

        // Search for Vorbis comment signatures (OpusTags or \x03vorbis)
        if let Some(pos) = Self::find_subsequence(bytes, b"OpusTags").or_else(|| Self::find_subsequence(bytes, b"\x03vorbis")) {
            let chunk = &bytes[pos..std::cmp::min(pos + 4096, bytes.len())];
            Self::extract_comment_fields(chunk, &mut meta);
        }

        meta
    }

    fn extract_comment_fields(chunk: &[u8], meta: &mut AudioMetadata) {
        let text = String::from_utf8_lossy(chunk);
        for line in text.split('\0').flat_map(|s| s.lines()) {
            if let Some((k, v)) = line.split_once('=') {
                let key = k.trim().to_uppercase();
                let val = v.trim().to_string();
                if val.is_empty() {
                    continue;
                }
                match key.as_str() {
                    "TITLE" if meta.title.is_none() => meta.title = Some(val),
                    "ARTIST" if meta.artist.is_none() => meta.artist = Some(val),
                    "ALBUM" if meta.album.is_none() => meta.album = Some(val),
                    "GENRE" if meta.genre.is_none() => meta.genre = Some(val),
                    "DATE" if meta.year.is_none() => {
                        if let Ok(y) = val[..std::cmp::min(4, val.len())].parse::<u32>() {
                            meta.year = Some(y);
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    fn find_subsequence(haystack: &[u8], needle: &[u8]) -> Option<usize> {
        haystack.windows(needle.len()).position(|window| window == needle)
    }
}

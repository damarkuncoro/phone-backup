use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AudioFormat {
    Mp3,
    Opus,
    Ogg,
    Aac,
    M4a,
    Flac,
    Wav,
    Amr,
    Unknown,
}

impl AudioFormat {
    pub fn from_magic_or_extension(bytes: &[u8], extension: &str) -> Self {
        if bytes.len() >= 4 {
            if bytes.starts_with(b"ID3") || (bytes[0] == 0xFF && (bytes[1] & 0xE0) == 0xE0) {
                return Self::Mp3;
            }
            if bytes.starts_with(b"OggS") {
                if bytes.len() >= 36 && &bytes[28..36] == b"OpusHead" {
                    return Self::Opus;
                }
                return Self::Ogg;
            }
            if bytes.len() >= 8 && &bytes[4..8] == b"ftyp" {
                return Self::M4a;
            }
            if bytes.starts_with(b"fLaC") {
                return Self::Flac;
            }
            if bytes.starts_with(b"RIFF") {
                return Self::Wav;
            }
            if bytes.starts_with(b"#!AMR") {
                return Self::Amr;
            }
        }

        match extension.to_lowercase().as_str() {
            "mp3" => Self::Mp3,
            "opus" => Self::Opus,
            "ogg" => Self::Ogg,
            "m4a" => Self::M4a,
            "aac" => Self::Aac,
            "flac" => Self::Flac,
            "wav" => Self::Wav,
            "amr" => Self::Amr,
            _ => Self::Unknown,
        }
    }
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AudioCategory {
    Music,
    WhatsAppVoiceNote,
    CallRecording,
    VoiceMemo,
    Ringtone,
    Unknown,
}

impl AudioCategory {
    pub fn is_voice_content(&self) -> bool {
        matches!(self, Self::WhatsAppVoiceNote | Self::CallRecording | Self::VoiceMemo)
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::Music => "Music Track",
            Self::WhatsAppVoiceNote => "WhatsApp Voice Note (PTT)",
            Self::CallRecording => "Call Recording",
            Self::VoiceMemo => "Voice Memo",
            Self::Ringtone => "Ringtone / Notification",
            Self::Unknown => "Audio",
        }
    }
}

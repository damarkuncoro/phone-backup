use crate::model::AudioCategory;

pub struct AudioClassifier;

impl AudioClassifier {
    /// Classifies an audio file based on relative path and filename.
    pub fn classify(rel_path: &str, filename: &str) -> AudioCategory {
        let lower_path = rel_path.to_lowercase();
        let lower_name = filename.to_lowercase();

        // 1. WhatsApp Voice Notes (PTT)
        if lower_path.contains("whatsapp voice notes") || lower_name.starts_with("ptt-") {
            return AudioCategory::WhatsAppVoiceNote;
        }

        // 2. Call Recordings
        if lower_path.contains("call") || lower_path.contains("recordings/call")
            || lower_name.starts_with("call@") || lower_name.starts_with("call_")
        {
            return AudioCategory::CallRecording;
        }

        // 3. Voice Memos / Dictaphone
        if lower_path.contains("voicenotes") || lower_path.contains("sound_recorder")
            || lower_name.starts_with("voice_") || lower_name.starts_with("rec_")
        {
            return AudioCategory::VoiceMemo;
        }

        // 4. Ringtone / Notifications
        if lower_path.contains("ringtones") || lower_path.contains("notifications") || lower_path.contains("alarms") {
            return AudioCategory::Ringtone;
        }

        // 5. Default to Music
        AudioCategory::Music
    }
}

use phone_backup_audio::{
    AudioClassifier, AudioCategory, CallDirection, CallRecordingParser,
};

#[test]
fn test_classify_whatsapp_ptt_and_call_recording() {
    let cat_ptt = AudioClassifier::classify("WhatsApp/Media/WhatsApp Voice Notes/202328", "PTT-20230715-WA0000.opus");
    assert_eq!(cat_ptt, AudioCategory::WhatsAppVoiceNote);
    assert!(cat_ptt.is_voice_content());

    let cat_call = AudioClassifier::classify("Recordings/Call", "Call@+628123456789_(2023-05-12_14.30.22)_in.m4a");
    assert_eq!(cat_call, AudioCategory::CallRecording);

    let cat_music = AudioClassifier::classify("Music/Rock", "Queen - Bohemian Rhapsody.mp3");
    assert_eq!(cat_music, AudioCategory::Music);
}

#[test]
fn test_parse_call_recording_filename() {
    let filename = "Call@+628123456789_(2023-05-12_14.30.22)_in.m4a";
    let info = CallRecordingParser::parse_filename(filename);

    assert_eq!(info.phone_number.as_deref(), Some("+628123456789"));
    assert_eq!(info.direction, CallDirection::Incoming);
    assert!(info.timestamp.is_some());
    assert_eq!(info.timestamp.unwrap().format("%Y-%m-%d %H:%M:%S").to_string(), "2023-05-12 14:30:22");
}

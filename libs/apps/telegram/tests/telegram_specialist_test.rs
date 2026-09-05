use phone_backup_telegram::{
    ChatType, TelegramChatBuilder, TelegramChatIndexer, TelegramFactory, TelegramHtmlExporter,
    TelegramJsonExporter, TelegramJsonParser, TelegramMediaType, TelegramMessageBuilder,
    TelegramPathResolver,
};

#[test]
fn test_media_type_and_path_resolver() {
    assert_eq!(
        TelegramPathResolver::classify_path("/sdcard/Telegram/Telegram Audio/voice_1.ogg"),
        TelegramMediaType::VoiceNote
    );
    assert_eq!(
        TelegramPathResolver::classify_path("/sdcard/Telegram/Telegram Video/video_note.mp4"),
        TelegramMediaType::VideoNote
    );
    assert_eq!(
        TelegramPathResolver::classify_path("/sdcard/Telegram/Telegram Images/photo.jpg"),
        TelegramMediaType::Photo
    );
    assert_eq!(
        TelegramPathResolver::classify_path("/sdcard/Telegram/Telegram Documents/contract.pdf"),
        TelegramMediaType::Document
    );
}

#[test]
fn test_builder_and_factory() {
    let msg1 = TelegramFactory::create_text_message(1, "Alice", "Hello World!");
    assert_eq!(msg1.id, 1);
    assert_eq!(msg1.sender_name.as_deref(), Some("Alice"));
    assert_eq!(msg1.text, "Hello World!");
    assert_eq!(msg1.media_type, TelegramMediaType::TextOnly);

    let voice_msg = TelegramFactory::create_voice_note(
        2,
        "Bob",
        "voice_messages/audio_1.ogg",
        15,
    );
    assert_eq!(voice_msg.media_type, TelegramMediaType::VoiceNote);
    assert_eq!(voice_msg.duration_secs, Some(15));

    let built_msg = TelegramMessageBuilder::new()
        .id(3)
        .sender("Charlie")
        .media(TelegramMediaType::VideoNote, "videos/round.mp4")
        .duration_secs(20)
        .build()
        .expect("Build should succeed");

    assert_eq!(built_msg.id, 3);
    assert_eq!(built_msg.media_type, TelegramMediaType::VideoNote);

    let chat = TelegramChatBuilder::new()
        .id(100)
        .title("Project Alpha")
        .chat_type(ChatType::Supergroup)
        .add_message(msg1)
        .add_message(voice_msg)
        .add_message(built_msg)
        .build()
        .expect("Chat build should succeed");

    assert_eq!(chat.total_messages(), 3);
    assert_eq!(chat.total_media_messages(), 2);
}

#[test]
fn test_json_export_parser() {
    let json = r#"{
  "name": "Dev Chat",
  "type": "public_supergroup",
  "id": 9999,
  "messages": [
    {
      "id": 1,
      "type": "message",
      "date": "2026-09-05T10:00:00",
      "from": "Dave",
      "from_id": "user99",
      "text": "Check this update"
    },
    {
      "id": 2,
      "type": "message",
      "date": "2026-09-05T10:05:00",
      "from": "Eve",
      "from_id": "user100",
      "text": "",
      "media_type": "voice_message",
      "file": "voice_messages/audio_2.ogg",
      "duration_seconds": 8
    }
  ]
}"#;

    let chat = TelegramJsonParser::parse(json).expect("JSON parsing should succeed");
    assert_eq!(chat.id, 9999);
    assert_eq!(chat.title, "Dev Chat");
    assert_eq!(chat.chat_type, ChatType::Supergroup);
    assert_eq!(chat.messages.len(), 2);
    assert_eq!(chat.messages[0].sender_name.as_deref(), Some("Dave"));
    assert_eq!(chat.messages[1].media_type, TelegramMediaType::VoiceNote);
    assert_eq!(chat.messages[1].duration_secs, Some(8));
}

#[test]
fn test_chat_indexer_and_exporter() {
    let msg1 = TelegramFactory::create_text_message(1, "Alice", "Meeting starts");
    let msg2 = TelegramFactory::create_voice_note(2, "Bob", "voice.ogg", 10);
    let mut chat = TelegramFactory::create_chat(1, "Standup", ChatType::Group);
    chat.add_message(msg1);
    chat.add_message(msg2);

    let stats = TelegramChatIndexer::compute_media_stats(&chat);
    assert_eq!(stats.get(&TelegramMediaType::TextOnly), Some(&1));
    assert_eq!(stats.get(&TelegramMediaType::VoiceNote), Some(&1));

    let html = TelegramHtmlExporter::export(&chat);
    assert!(html.contains("Standup"));
    assert!(html.contains("Meeting starts"));
    assert!(html.contains("Voice Note"));

    let json = TelegramJsonExporter::export_pretty(&chat).expect("JSON export should succeed");
    assert!(json.contains("Standup"));
}

use phone_backup_audio::{
    AudioCategory, AudioFormat, AudioPipelineBuilder,
};

#[test]
fn test_audio_pipeline_builder_processing() {
    let mut mp3_bytes = vec![0u8; 500];
    let mut id3v1 = [0u8; 128];
    id3v1[0..3].copy_from_slice(b"TAG");
    id3v1[3..10].copy_from_slice(b"My Song");
    mp3_bytes[500 - 128..].copy_from_slice(&id3v1);

    let pipeline = AudioPipelineBuilder::new().with_waveform_points(50);
    let result = pipeline
        .process("Music/Pop/track01.mp3", &mp3_bytes)
        .expect("Pipeline processing failed");

    assert_eq!(result.format, AudioFormat::Mp3);
    assert_eq!(result.category, AudioCategory::Music);
    assert_eq!(result.metadata.title.as_deref(), Some("My Song"));
    assert_eq!(result.waveform.count(), 50);
}

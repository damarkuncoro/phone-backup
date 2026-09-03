use phone_backup_audio::WaveformGenerator;

#[test]
fn test_waveform_peak_generation() {
    let mut audio_bytes = Vec::new();
    // Simulate sinusoidal-like audio bytes
    for i in 0..10_000 {
        let val = (128.0 + 100.0 * ((i as f64) * 0.05).sin()) as u8;
        audio_bytes.push(val);
    }

    let waveform = WaveformGenerator::generate_peaks(&audio_bytes, 100);

    assert_eq!(waveform.count(), 100);
    assert!(!waveform.is_empty());
    // Peaks should have positive normalized values
    assert!(waveform.points.iter().any(|&p| p > 50));
}

use audio_engine::AudioPipelineBuilder;
use image_engine::BlurDetector;
use std::fs;

#[derive(serde::Serialize)]
pub struct AudioAnalysisResult {
    pub format: String,
    pub category: String,
    pub title: Option<String>,
    pub artist: Option<String>,
    pub waveform_points: Vec<u8>,
}

#[tauri::command(rename_all = "snake_case")]
pub async fn analyze_audio_file(path: String) -> Result<AudioAnalysisResult, String> {
    let bytes = fs::read(&path).map_err(|e| e.to_string())?;
    let pipeline = AudioPipelineBuilder::new().with_waveform_points(60);
    let processed = pipeline.process(&path, &bytes).map_err(|e| e.to_string())?;

    Ok(AudioAnalysisResult {
        format: format!("{:?}", processed.format),
        category: processed.category.label().to_string(),
        title: processed.metadata.title,
        artist: processed.metadata.artist,
        waveform_points: processed.waveform.points,
    })
}

#[tauri::command(rename_all = "snake_case")]
pub async fn check_image_sharpness(path: String) -> Result<f64, String> {
    let bytes = fs::read(&path).map_err(|e| e.to_string())?;
    let dyn_img = image::load_from_memory(&bytes).map_err(|e| e.to_string())?;
    Ok(BlurDetector::compute_sharpness(&dyn_img))
}

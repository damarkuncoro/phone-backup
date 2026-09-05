use crate::intelligence::{AudioClassifier, CallRecordingInfo, CallRecordingParser};
use crate::model::{AudioCategory, AudioFormat, AudioMetadata, WaveformPeaks};
use crate::parser::{Id3Reader, OggVorbisReader};
use crate::waveform::WaveformGenerator;
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct ProcessedAudio {
    pub metadata: AudioMetadata,
    pub format: AudioFormat,
    pub category: AudioCategory,
    pub call_info: Option<CallRecordingInfo>,
    pub waveform: WaveformPeaks,
}

pub struct AudioPipelineBuilder {
    waveform_points: usize,
    extract_call_info: bool,
}

impl AudioPipelineBuilder {
    pub fn new() -> Self {
        Self {
            waveform_points: WaveformGenerator::DEFAULT_POINTS,
            extract_call_info: true,
        }
    }

    pub fn with_waveform_points(mut self, points: usize) -> Self {
        self.waveform_points = points;
        self
    }

    pub fn process(&self, rel_path: &str, bytes: &[u8]) -> Result<ProcessedAudio> {
        let filename = rel_path.split(['/', '\\']).next_back().unwrap_or(rel_path);
        let ext = filename.split('.').next_back().unwrap_or("");

        let format = AudioFormat::from_magic_or_extension(bytes, ext);
        let category = AudioClassifier::classify(rel_path, filename);

        let mut metadata = match format {
            AudioFormat::Mp3 => Id3Reader::read_tags(bytes),
            AudioFormat::Ogg | AudioFormat::Opus => OggVorbisReader::read_tags(bytes),
            _ => AudioMetadata::new(),
        };

        metadata.format = Some(format);
        metadata.category = Some(category);

        let call_info = if self.extract_call_info && category == AudioCategory::CallRecording {
            Some(CallRecordingParser::parse_filename(filename))
        } else {
            None
        };

        let waveform = WaveformGenerator::generate_peaks(bytes, self.waveform_points);

        Ok(ProcessedAudio {
            metadata,
            format,
            category,
            call_info,
            waveform,
        })
    }
}

impl Default for AudioPipelineBuilder {
    fn default() -> Self {
        Self::new()
    }
}

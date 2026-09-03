use super::smart_engine::SmartCompressionEngine;
use crate::analysis::entropy::EntropyDetector;
use crate::analysis::probe::SampleProbe;
use crate::config::{CompressionAlgorithm, CompressionPolicy};
use crate::dict::manager::DictionaryManager;

/// Fluent Builder for constructing configured `SmartCompressionEngine` instances.
#[derive(Debug, Clone)]
pub struct CompressionEngineBuilder {
    pub(crate) policy: CompressionPolicy,
    pub(crate) default_algorithm: CompressionAlgorithm,
    pub(crate) entropy_threshold: f64,
    pub(crate) enable_entropy_check: bool,
    pub(crate) enable_sample_probe: bool,
    pub(crate) sample_size: usize,
    pub(crate) min_saving_ratio: f64,
    pub(crate) chunk_size: usize,
    pub(crate) dictionary_manager: Option<DictionaryManager>,
}

impl Default for CompressionEngineBuilder {
    fn default() -> Self {
        Self {
            policy: CompressionPolicy::Balanced,
            default_algorithm: CompressionAlgorithm::Zstd,
            entropy_threshold: EntropyDetector::DEFAULT_HIGH_ENTROPY_THRESHOLD,
            enable_entropy_check: true,
            enable_sample_probe: true,
            sample_size: SampleProbe::DEFAULT_SAMPLE_SIZE,
            min_saving_ratio: 0.05,
            chunk_size: 4 * 1024 * 1024,
            dictionary_manager: None,
        }
    }
}

impl CompressionEngineBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_policy(mut self, policy: CompressionPolicy) -> Self {
        self.policy = policy;
        self
    }

    pub fn with_default_algorithm(mut self, algorithm: CompressionAlgorithm) -> Self {
        self.default_algorithm = algorithm;
        self
    }

    pub fn with_entropy_threshold(mut self, threshold: f64) -> Self {
        self.entropy_threshold = threshold;
        self
    }

    pub fn with_entropy_check(mut self, enabled: bool) -> Self {
        self.enable_entropy_check = enabled;
        self
    }

    pub fn with_sample_probe(mut self, enabled: bool) -> Self {
        self.enable_sample_probe = enabled;
        self
    }

    pub fn with_sample_size(mut self, sample_size: usize) -> Self {
        self.sample_size = sample_size;
        self
    }

    pub fn with_min_saving_ratio(mut self, min_saving: f64) -> Self {
        self.min_saving_ratio = min_saving;
        self
    }

    pub fn with_chunk_size(mut self, chunk_size: usize) -> Self {
        self.chunk_size = chunk_size;
        self
    }

    pub fn with_dictionary_manager(mut self, manager: DictionaryManager) -> Self {
        self.dictionary_manager = Some(manager);
        self
    }

    pub fn with_android_dictionaries(mut self) -> Self {
        self.dictionary_manager = Some(DictionaryManager::with_android_defaults());
        self
    }

    pub fn build(self) -> SmartCompressionEngine {
        SmartCompressionEngine::from_builder(self)
    }
}

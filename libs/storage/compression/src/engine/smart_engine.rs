use super::builder::CompressionEngineBuilder;
use crate::analysis::classifier::{ContentClassifier, DataCategory};
use crate::analysis::entropy::EntropyDetector;
use crate::analysis::probe::SampleProbe;
use crate::config::{
    CompressionAlgorithm, CompressionDecision, CompressionLevel, CompressionPolicy,
    FileMetadataContext,
};
use crate::dict::models::{CompressionDictionary, DictionaryId};
use crate::stats::CompressionStats;
use crate::strategies::factory::CompressionStrategyFactory;
use anyhow::Result;
use std::sync::Arc;
use std::time::Instant;

/// Adaptive compression engine tailored for phone backup workloads.
pub struct SmartCompressionEngine {
    builder: CompressionEngineBuilder,
}

impl SmartCompressionEngine {
    pub fn builder() -> CompressionEngineBuilder {
        CompressionEngineBuilder::default()
    }

    pub(crate) fn from_builder(builder: CompressionEngineBuilder) -> Self {
        Self { builder }
    }

    /// Evaluates input data and metadata context to make an intelligent compression decision.
    pub fn plan(&self, data: &[u8], context: &FileMetadataContext) -> CompressionDecision {
        if self.builder.policy == CompressionPolicy::None {
            return self.disabled_decision("Policy explicitly set to None");
        }

        // 1. Check classification (magic bytes + metadata)
        let category = self.detect_category(data, context);
        if ContentClassifier::is_precompressed(category) {
            return self.disabled_decision(format!("Pre-compressed format ({category:?})"));
        }

        // 2. Check Entropy on sample
        if self.builder.enable_entropy_check && data.len() >= 1024 {
            let entropy = EntropyDetector::sample_entropy(data, self.builder.sample_size);
            if entropy >= self.builder.entropy_threshold {
                return self.disabled_decision(format!("High entropy ({entropy:.2})"));
            }
        }

        // 3. Probing for adaptive policy
        if self.builder.policy == CompressionPolicy::Adaptive
            && self.builder.enable_sample_probe
            && data.len() >= 32 * 1024
            && !SampleProbe::is_worth_compressing(
                data,
                self.builder.sample_size,
                self.builder.min_saving_ratio,
            )
        {
            return self.disabled_decision("Sample probe showed negligible savings");
        }

        let dict_id = self.resolve_dictionary(category).map(|d| d.id.clone());
        let level = self.resolve_level(category);

        CompressionDecision {
            algorithm: self.builder.default_algorithm,
            level,
            dictionary_id: dict_id,
            chunk_size: self.builder.chunk_size,
            enabled: true,
            reason: format!("Compressible ({category:?}) level {level:?}"),
        }
    }

    /// Compresses data based on planned decision and returns compressed bytes with stats.
    pub fn compress(
        &self,
        data: &[u8],
        context: &FileMetadataContext,
    ) -> Result<(Vec<u8>, CompressionStats)> {
        let decision = self.plan(data, context);
        let orig_len = data.len() as u64;

        if !decision.enabled || decision.algorithm == CompressionAlgorithm::None {
            return Ok((data.to_vec(), CompressionStats::no_compression(orig_len)));
        }

        let dict = decision
            .dictionary_id
            .as_ref()
            .and_then(|id| self.get_dictionary(id));
        let strategy = match dict {
            Some(d) => CompressionStrategyFactory::create_with_dictionary(
                decision.algorithm,
                decision.level,
                (*d).clone(),
            ),
            None => CompressionStrategyFactory::create(decision.algorithm, decision.level),
        };

        let start = Instant::now();
        let compressed = strategy.compress(data)?;
        let duration_us = start.elapsed().as_micros();

        if compressed.len() < data.len() {
            let stats = CompressionStats::new(
                decision.algorithm,
                orig_len,
                compressed.len() as u64,
                duration_us,
            );
            Ok((compressed, stats))
        } else {
            Ok((data.to_vec(), CompressionStats::no_compression(orig_len)))
        }
    }

    pub fn decompress(&self, data: &[u8], algorithm: CompressionAlgorithm) -> Result<Vec<u8>> {
        CompressionStrategyFactory::create_default(algorithm).decompress(data)
    }

    pub fn decompress_with_dictionary(
        &self,
        data: &[u8],
        algorithm: CompressionAlgorithm,
        dictionary: CompressionDictionary,
    ) -> Result<Vec<u8>> {
        let strat = CompressionStrategyFactory::create_with_dictionary(
            algorithm,
            CompressionLevel::default(),
            dictionary,
        );
        strat.decompress(data)
    }

    fn detect_category(&self, data: &[u8], context: &FileMetadataContext) -> DataCategory {
        let magic_cat = ContentClassifier::classify_magic_bytes(data);
        if magic_cat != DataCategory::Unknown {
            return magic_cat;
        }
        if let Some(ext) = &context.extension {
            let cat = ContentClassifier::classify_extension(ext);
            if cat != DataCategory::Unknown {
                return cat;
            }
        }
        if let Some(mime) = &context.mime_type {
            let cat = ContentClassifier::classify_mime(mime);
            if cat != DataCategory::Unknown {
                return cat;
            }
        }
        DataCategory::Unknown
    }

    fn resolve_level(&self, category: DataCategory) -> CompressionLevel {
        match self.builder.policy {
            CompressionPolicy::None | CompressionPolicy::Fast => CompressionLevel::Fast,
            CompressionPolicy::Balanced => CompressionLevel::Balanced,
            CompressionPolicy::Maximum => CompressionLevel::Maximum,
            CompressionPolicy::Adaptive => {
                if ContentClassifier::is_highly_compressible(category) {
                    CompressionLevel::High
                } else {
                    CompressionLevel::Balanced
                }
            }
        }
    }

    fn resolve_dictionary(&self, category: DataCategory) -> Option<Arc<CompressionDictionary>> {
        self.builder
            .dictionary_manager
            .as_ref()?
            .get_by_category(category)
    }

    pub fn get_dictionary(&self, id: &DictionaryId) -> Option<Arc<CompressionDictionary>> {
        self.builder.dictionary_manager.as_ref()?.get_by_id(id)
    }

    fn disabled_decision(&self, reason: impl Into<String>) -> CompressionDecision {
        CompressionDecision {
            algorithm: CompressionAlgorithm::None,
            level: CompressionLevel::Fast,
            dictionary_id: None,
            chunk_size: self.builder.chunk_size,
            enabled: false,
            reason: reason.into(),
        }
    }
}

//! Specialist Device Scanner Library.
//!
//! Provides enterprise-grade device scanning, noise filtering, category classification,
//! multi-source merging, incremental diffing, builder/factory patterns, and scan performance metrics.

pub mod builder;
pub mod classifier;
pub mod factory;
pub mod incremental;
pub mod merger;
pub mod metrics;
pub mod noise_filter;
pub mod pipeline;

pub use builder::{ScanFilterBuilder, ScanPipelineBuilder};
pub use classifier::FileClassifier;
pub use factory::ScanPipelineFactory;
pub use incremental::IncrementalScanner;
pub use merger::FileMerger;
pub use metrics::ScanMetricsTracker;
pub use noise_filter::NoiseFilter;
pub use pipeline::ScanPipeline;

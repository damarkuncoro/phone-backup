use crate::pipeline::ScanPipeline;
use domain::{ScanFilter, ScanWarning};

/// Fluent builder for creating fine-grained `ScanFilter` configurations.
#[derive(Debug, Clone, Default)]
pub struct ScanFilterBuilder {
    filter: ScanFilter,
}

impl ScanFilterBuilder {
    pub fn new() -> Self {
        Self {
            filter: ScanFilter::default(),
        }
    }

    pub fn exclude_noise(mut self, exclude: bool) -> Self {
        self.filter.exclude_noise = exclude;
        self
    }

    pub fn exclude_thumbnails(mut self, exclude: bool) -> Self {
        self.filter.exclude_thumbnails = exclude;
        self
    }

    pub fn exclude_cache(mut self, exclude: bool) -> Self {
        self.filter.exclude_cache = exclude;
        self
    }

    pub fn exclude_trash(mut self, exclude: bool) -> Self {
        self.filter.exclude_trash = exclude;
        self
    }

    pub fn exclude_nomedia(mut self, exclude: bool) -> Self {
        self.filter.exclude_nomedia = exclude;
        self
    }

    pub fn min_size_bytes(mut self, bytes: u64) -> Self {
        self.filter.min_size_bytes = Some(bytes);
        self
    }

    pub fn max_size_bytes(mut self, bytes: u64) -> Self {
        self.filter.max_size_bytes = Some(bytes);
        self
    }

    pub fn add_exclude_glob(mut self, glob: impl Into<String>) -> Self {
        self.filter.custom_exclude_globs.push(glob.into());
        self
    }

    pub fn build(self) -> ScanFilter {
        self.filter
    }
}

/// Fluent builder for constructing fully configured `ScanPipeline` instances.
#[derive(Default)]
pub struct ScanPipelineBuilder {
    filter: ScanFilter,
    dir_count: usize,
    warnings: Vec<ScanWarning>,
}

impl ScanPipelineBuilder {
    pub fn new() -> Self {
        Self {
            filter: ScanFilter::default(),
            dir_count: 0,
            warnings: Vec::new(),
        }
    }

    pub fn with_filter(mut self, filter: ScanFilter) -> Self {
        self.filter = filter;
        self
    }

    pub fn with_filter_builder(mut self, builder: ScanFilterBuilder) -> Self {
        self.filter = builder.build();
        self
    }

    pub fn with_directory_count(mut self, count: usize) -> Self {
        self.dir_count = count;
        self
    }

    pub fn with_warning(mut self, warning: ScanWarning) -> Self {
        self.warnings.push(warning);
        self
    }

    pub fn with_warnings(mut self, warnings: Vec<ScanWarning>) -> Self {
        self.warnings.extend(warnings);
        self
    }

    pub fn build(self) -> ScanPipeline {
        let mut pipeline = ScanPipeline::new(self.filter);
        if self.dir_count > 0 {
            pipeline.add_directory_count(self.dir_count);
        }
        pipeline.add_warnings(self.warnings);
        pipeline
    }
}

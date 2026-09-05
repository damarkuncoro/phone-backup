use crate::classifier::FileClassifier;
use crate::merger::FileMerger;
use crate::metrics::ScanMetricsTracker;
use crate::noise_filter::NoiseFilter;
use domain::{FileEntry, ScanFilter, ScanResult, ScanWarning};

/// Unified orchestrator for filtering, merging, classifying, and reporting scan results.
pub struct ScanPipeline {
    filter: ScanFilter,
    tracker: ScanMetricsTracker,
    warnings: Vec<ScanWarning>,
}

impl ScanPipeline {
    pub fn new(filter: ScanFilter) -> Self {
        Self {
            filter,
            tracker: ScanMetricsTracker::start(),
            warnings: Vec::new(),
        }
    }

    /// Creates a fluent `ScanPipelineBuilder`.
    pub fn builder() -> crate::builder::ScanPipelineBuilder {
        crate::builder::ScanPipelineBuilder::new()
    }

    /// Convenience DRY helper to process a single source with filter, directory count, and warnings.
    pub fn process_source(
        files: Vec<FileEntry>,
        dir_count: usize,
        filter: Option<&ScanFilter>,
        warnings: Vec<ScanWarning>,
    ) -> ScanResult {
        let mut pipeline = Self::new(filter.cloned().unwrap_or_default());
        pipeline.add_directory_count(if dir_count == 0 { 1 } else { dir_count });
        pipeline.add_warnings(warnings);
        pipeline.process_single_source(files)
    }

    pub fn add_directory_count(&mut self, count: usize) {
        self.tracker.add_directories(count);
    }

    pub fn add_warning(&mut self, warning: ScanWarning) {
        self.warnings.push(warning);
    }

    pub fn add_warnings(&mut self, warnings: Vec<ScanWarning>) {
        self.warnings.extend(warnings);
    }

    pub fn process_single_source(&mut self, files: Vec<FileEntry>) -> ScanResult {
        let filtered: Vec<FileEntry> = files
            .into_iter()
            .filter(|f| !NoiseFilter::should_ignore(&f.path, f.size_bytes, &self.filter))
            .collect();

        self.tracker.set_files_scanned(filtered.len());
        let categories = FileClassifier::summarize(&filtered);
        let metrics = self.tracker.finish();

        ScanResult::with_details(filtered, self.warnings.clone(), categories, Some(metrics))
    }

    pub fn process_multi_source(
        &mut self,
        primary: Vec<FileEntry>,
        secondary: Vec<FileEntry>,
    ) -> ScanResult {
        let filtered_primary: Vec<FileEntry> = primary
            .into_iter()
            .filter(|f| !NoiseFilter::should_ignore(&f.path, f.size_bytes, &self.filter))
            .collect();

        let filtered_secondary: Vec<FileEntry> = secondary
            .into_iter()
            .filter(|f| !NoiseFilter::should_ignore(&f.path, f.size_bytes, &self.filter))
            .collect();

        let merged_map = FileMerger::merge_collections(filtered_primary, filtered_secondary);
        let files: Vec<FileEntry> = merged_map.into_values().collect();

        self.tracker.set_files_scanned(files.len());
        let categories = FileClassifier::summarize(&files);
        let metrics = self.tracker.finish();

        ScanResult::with_details(files, self.warnings.clone(), categories, Some(metrics))
    }
}

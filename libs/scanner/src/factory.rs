use crate::builder::{ScanFilterBuilder, ScanPipelineBuilder};
use crate::pipeline::ScanPipeline;
use domain::ScanWarning;

/// Factory for creating pre-configured `ScanPipeline` and `ScanFilter` instances for specific targets.
pub struct ScanPipelineFactory;

impl ScanPipelineFactory {
    /// Factory method for Android ADB scanner with standard junk rejection and root diagnostics.
    pub fn for_android_adb(roots_count: usize, warnings: Vec<ScanWarning>) -> ScanPipeline {
        let filter = ScanFilterBuilder::new()
            .exclude_noise(true)
            .exclude_cache(true)
            .exclude_thumbnails(true)
            .exclude_trash(true)
            .exclude_nomedia(true)
            .build();

        ScanPipelineBuilder::new()
            .with_filter(filter)
            .with_directory_count(roots_count)
            .with_warnings(warnings)
            .build()
    }

    /// Factory method for MTP USB devices.
    pub fn for_mtp_usb(roots_count: usize) -> ScanPipeline {
        let filter = ScanFilterBuilder::new()
            .exclude_noise(true)
            .exclude_cache(true)
            .exclude_thumbnails(true)
            .build();

        ScanPipelineBuilder::new()
            .with_filter(filter)
            .with_directory_count(roots_count)
            .build()
    }

    /// Factory method for Apple iOS AFC (Apple File Conduit) media scanner.
    pub fn for_ios_afc() -> ScanPipeline {
        let filter = ScanFilterBuilder::new()
            .exclude_noise(false)
            .exclude_nomedia(false)
            .exclude_thumbnails(true)
            .exclude_cache(true)
            .build();

        ScanPipelineBuilder::new()
            .with_filter(filter)
            .with_directory_count(1)
            .build()
    }

    /// Factory method for Local Filesystem crawlers.
    pub fn for_local_filesystem(roots_count: usize) -> ScanPipeline {
        let filter = ScanFilterBuilder::new()
            .exclude_noise(true)
            .exclude_cache(true)
            .exclude_trash(true)
            .build();

        ScanPipelineBuilder::new()
            .with_filter(filter)
            .with_directory_count(roots_count)
            .build()
    }

    /// Factory method for Wireless Companion Agent streams.
    pub fn for_wireless_agent() -> ScanPipeline {
        let filter = ScanFilterBuilder::new()
            .exclude_noise(true)
            .exclude_cache(true)
            .build();

        ScanPipelineBuilder::new()
            .with_filter(filter)
            .with_directory_count(1)
            .build()
    }

    /// Factory method for raw deep scan with zero noise filtration.
    pub fn raw_deep_scan(roots_count: usize) -> ScanPipeline {
        let filter = ScanFilterBuilder::new()
            .exclude_noise(false)
            .exclude_thumbnails(false)
            .exclude_cache(false)
            .exclude_trash(false)
            .exclude_nomedia(false)
            .build();

        ScanPipelineBuilder::new()
            .with_filter(filter)
            .with_directory_count(roots_count)
            .build()
    }
}

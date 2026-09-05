use domain::ScanFilter;

/// High-performance noise and junk file filter for Android file structures.
pub struct NoiseFilter;

impl NoiseFilter {
    /// Evaluates whether a file path or directory should be ignored based on the scan filter.
    pub fn should_ignore(path: &str, size_bytes: u64, filter: &ScanFilter) -> bool {
        let normalized = path.replace('\\', "/");
        let lower = normalized.to_lowercase();

        if filter.exclude_noise && (Self::is_android_system_noise(&lower) || Self::is_vendor_junk(&lower)) {
            return true;
        }

        if filter.exclude_thumbnails && Self::is_thumbnail_path(&lower) {
            return true;
        }

        if filter.exclude_cache && Self::is_cache_path(&lower) {
            return true;
        }

        if filter.exclude_trash && Self::is_trash_path(&lower) {
            return true;
        }

        if filter.exclude_nomedia && lower.contains("/.nomedia") {
            return true;
        }

        if let Some(min) = filter.min_size_bytes {
            if size_bytes < min {
                return true;
            }
        }

        if let Some(max) = filter.max_size_bytes {
            if size_bytes > max {
                return true;
            }
        }

        if !filter.custom_exclude_globs.is_empty() {
            for glob in &filter.custom_exclude_globs {
                if lower.contains(&glob.to_lowercase()) {
                    return true;
                }
            }
        }

        false
    }

    fn is_thumbnail_path(lower: &str) -> bool {
        lower.contains("/.thumbnails/")
            || lower.contains("/.thumb/")
            || lower.ends_with("/.thumbnails")
            || lower.contains("/thumb_cache/")
    }

    fn is_cache_path(lower: &str) -> bool {
        lower.contains("/.cache/")
            || lower.contains("/cache/")
            || lower.contains("/code_cache/")
            || lower.contains("/cache_data/")
            || lower.ends_with(".cache")
    }

    fn is_trash_path(lower: &str) -> bool {
        lower.contains("/.trash/")
            || lower.contains("/.recycle/")
            || lower.contains("/$recycle.bin/")
            || lower.contains("/.trashed-")
            || lower.contains("/lost+found/")
    }

    fn is_vendor_junk(lower: &str) -> bool {
        lower.contains("/.yuetu_unlock/")
            || lower.contains("/.vivo_theme/")
            || lower.contains("/.miui_cache/")
            || lower.contains("/.coloros/")
            || lower.contains("/.estrongs/")
            || lower.ends_with("/.clear_sdcard.ini")
            || lower.ends_with("/.sdcard_version")
    }

    fn is_android_system_noise(lower: &str) -> bool {
        lower.ends_with(".tmp")
            || lower.ends_with(".part")
            || lower.ends_with(".crdownload")
            || lower.ends_with(".download")
            || lower.ends_with(".temp")
            || lower.contains("/.temp/")
            || lower.ends_with(".ds_store")
            || lower.contains("/__macosx/")
    }
}

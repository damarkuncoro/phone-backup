use domain::{FileDiff, FileEntry};
use std::collections::{HashMap, HashSet};

/// High-speed change detector based on mtime and size comparison.
pub struct IncrementalScanner;

impl IncrementalScanner {
    /// Compares a list of scanned files against a previous snapshot index.
    pub fn diff(
        current_files: &[FileEntry],
        previous_index: &HashMap<String, FileEntry>,
    ) -> FileDiff {
        let mut added = Vec::new();
        let mut modified = Vec::new();
        let mut current_paths = HashSet::with_capacity(current_files.len());

        for current in current_files {
            current_paths.insert(&current.path);
            if let Some(prev) = previous_index.get(&current.path) {
                if prev.size_bytes != current.size_bytes || prev.modified_at != current.modified_at
                {
                    modified.push(current.clone());
                }
            } else {
                added.push(current.clone());
            }
        }

        let mut removed = Vec::new();
        for (prev_path, prev_file) in previous_index {
            if !current_paths.contains(prev_path) {
                removed.push(prev_file.clone());
            }
        }

        added.sort_by(|a, b| a.path.cmp(&b.path));
        modified.sort_by(|a, b| a.path.cmp(&b.path));
        removed.sort_by(|a, b| a.path.cmp(&b.path));

        FileDiff {
            added,
            removed,
            modified,
        }
    }

    /// Separates changed files (New or Modified) from unchanged ones.
    pub fn partition_changed(
        current_files: Vec<FileEntry>,
        previous_index: &HashMap<String, FileEntry>,
    ) -> (Vec<FileEntry>, Vec<FileEntry>) {
        let mut changed = Vec::new();
        let mut unchanged = Vec::new();

        for current in current_files {
            if let Some(prev) = previous_index.get(&current.path) {
                if prev.size_bytes == current.size_bytes && prev.modified_at == current.modified_at
                {
                    unchanged.push(current);
                } else {
                    changed.push(current);
                }
            } else {
                changed.push(current);
            }
        }

        (changed, unchanged)
    }
}

use domain::{FileDiff, FileEntry};
use std::collections::{HashMap, HashSet};

fn normalize_path(path: &str) -> String {
    let p = path.trim();
    if let Some(rest) = p.strip_prefix("/sdcard/") {
        format!("/storage/emulated/0/{}", rest)
    } else if let Some(rest) = p.strip_prefix("sdcard/") {
        format!("/storage/emulated/0/{}", rest)
    } else if let Some(rest) = p.strip_prefix("/storage/self/primary/") {
        format!("/storage/emulated/0/{}", rest)
    } else {
        p.to_string()
    }
}

/// High-speed change detector based on mtime and size comparison with Android path alias normalization.
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

        let mut norm_prev = HashMap::with_capacity(previous_index.len());
        for (k, v) in previous_index {
            norm_prev.insert(normalize_path(k), (k, v));
        }

        for current in current_files {
            let norm_cur = normalize_path(&current.path);
            current_paths.insert(norm_cur.clone());
            if let Some((_, prev)) = norm_prev.get(&norm_cur) {
                if prev.size_bytes != current.size_bytes || prev.modified_at != current.modified_at
                {
                    modified.push(current.clone());
                }
            } else {
                added.push(current.clone());
            }
        }

        let mut removed = Vec::new();
        for (norm_p, (_, prev_file)) in &norm_prev {
            if !current_paths.contains(norm_p) {
                removed.push((*prev_file).clone());
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

        let mut norm_prev = HashMap::with_capacity(previous_index.len());
        for (k, v) in previous_index {
            norm_prev.insert(normalize_path(k), v);
        }

        for current in current_files {
            let norm_cur = normalize_path(&current.path);
            if let Some(prev) = norm_prev.get(&norm_cur) {
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

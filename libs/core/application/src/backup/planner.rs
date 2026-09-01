use domain::{BackupPlan, DeletedFile, FileEntry, FileReuse};
use std::collections::{HashMap, HashSet};

/// Pure domain/application service that constructs a BackupPlan from manifest scanning and prior snapshot state.
pub struct BackupPlanner;

impl BackupPlanner {
    pub fn build_plan(
        manifest_files: &[FileEntry],
        previous_files: &HashMap<String, FileEntry>,
        already_backed_up: &HashSet<String>,
    ) -> BackupPlan {
        let mut upload = Vec::new();
        let mut reuse = Vec::new();

        let current_manifest_paths: HashSet<&str> = manifest_files.iter().map(|f| f.path.as_str()).collect();

        for file in manifest_files {
            if already_backed_up.contains(&file.path) {
                reuse.push(FileReuse {
                    path: file.path.clone(),
                    size_bytes: file.size_bytes,
                    hash_sha256: file.hash_sha256.clone(),
                });
                continue;
            }

            if let Some(prev) = previous_files.get(&file.path) {
                if prev.size_bytes == file.size_bytes && prev.modified_at == file.modified_at {
                    reuse.push(FileReuse {
                        path: file.path.clone(),
                        size_bytes: file.size_bytes,
                        hash_sha256: prev.hash_sha256.clone(),
                    });
                } else {
                    upload.push(file.clone());
                }
            } else {
                upload.push(file.clone());
            }
        }

        let mut deleted = Vec::new();
        for (prev_path, prev_file) in previous_files {
            if !current_manifest_paths.contains(prev_path.as_str()) {
                deleted.push(DeletedFile {
                    path: prev_path.clone(),
                    size_bytes: prev_file.size_bytes,
                });
            }
        }

        BackupPlan::new(upload, reuse, Vec::new(), deleted)
    }
}

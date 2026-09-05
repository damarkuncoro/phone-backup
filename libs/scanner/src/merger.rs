use domain::FileEntry;
use std::collections::BTreeMap;

/// Deterministic multi-source file entry merger with field-level priority resolution.
pub struct FileMerger;

impl FileMerger {
    /// Merges an entry from MediaStore and POSIX FileSystem, combining rich metadata.
    pub fn merge_entries(mediastore: FileEntry, filesystem: FileEntry) -> FileEntry {
        FileEntry {
            id: mediastore.id,
            device_id: filesystem.device_id,
            path: mediastore.path,
            name: filesystem.name,
            size_bytes: filesystem.size_bytes,
            modified_at: filesystem.modified_at,
            mime_type: if mediastore.mime_type.is_empty() {
                filesystem.mime_type
            } else {
                mediastore.mime_type
            },
            permissions: filesystem.permissions,
            hash_sha256: filesystem.hash_sha256.or(mediastore.hash_sha256),
            thumbnail_hash: mediastore.thumbnail_hash.or(filesystem.thumbnail_hash),
            media_info: mediastore.media_info.or(filesystem.media_info),
        }
    }

    /// Merges two collections into a deterministically ordered BTreeMap keyed by path.
    pub fn merge_collections(
        primary: Vec<FileEntry>,
        secondary: Vec<FileEntry>,
    ) -> BTreeMap<String, FileEntry> {
        let mut map = BTreeMap::new();
        for item in primary {
            map.insert(item.path.clone(), item);
        }
        for item in secondary {
            if let Some(existing) = map.remove(&item.path) {
                let merged = Self::merge_entries(existing, item);
                map.insert(merged.path.clone(), merged);
            } else {
                map.insert(item.path.clone(), item);
            }
        }
        map
    }
}

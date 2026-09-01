use serde::{Deserialize, Serialize};
use crate::{Snapshot, FileEntry};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manifest {
    pub snapshot: Snapshot,
    pub files: Vec<ManifestFile>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManifestFile {
    pub file: FileEntry,
    pub chunks: Vec<ManifestChunk>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManifestChunk {
    pub id: String,
    pub offset: u64,
    pub length: u32,
}

impl Manifest {
    pub fn new(snapshot: Snapshot, files: Vec<ManifestFile>) -> Self {
        Self { snapshot, files }
    }
}

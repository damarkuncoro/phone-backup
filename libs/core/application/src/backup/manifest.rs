use anyhow::Result;
use domain::{Manifest, ManifestChunk, ManifestFile, Snapshot};
use ports::{RepositoryPort, StoragePort};
use tracing::{info, instrument};

pub struct ManifestManager<'a, R: RepositoryPort, T: StoragePort> {
    repository: &'a R,
    storage: &'a T,
}

impl<'a, R: RepositoryPort, T: StoragePort> ManifestManager<'a, R, T> {
    pub fn new(repository: &'a R, storage: &'a T) -> Self {
        Self {
            repository,
            storage,
        }
    }

    #[instrument(skip(self, snapshot))]
    pub fn create_and_store_manifest(&self, snapshot: &Snapshot) -> Result<()> {
        info!(
            "Creating immutable manifest for snapshot: {}",
            snapshot.id.0
        );

        let files = self.repository.get_snapshot_files(&snapshot.id)?;
        let mut manifest_files = Vec::with_capacity(files.len());

        for file in files {
            let chunks_data = self.repository.get_file_chunks(&file.id)?;
            let chunks = chunks_data
                .into_iter()
                .map(|(id, offset, length, _)| ManifestChunk { id, offset, length })
                .collect();

            manifest_files.push(ManifestFile { file, chunks });
        }

        let manifest = Manifest::new(snapshot.clone(), manifest_files);
        let manifest_json = serde_json::to_vec(&manifest)?;

        let manifest_path = format!("manifests/{}.json", snapshot.id.0);
        self.storage
            .write(&manifest_path, &mut std::io::Cursor::new(manifest_json))?;

        info!("Manifest stored successfully at: {}", manifest_path);
        Ok(())
    }
}

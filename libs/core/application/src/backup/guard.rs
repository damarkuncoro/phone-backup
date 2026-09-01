use domain::{Snapshot, SnapshotStatus};
use ports::RepositoryPort;

/// RAII Guard that ensures a snapshot never remains stranded in `Running` status if error or panic occurs.
pub struct SnapshotGuard<'a, R: RepositoryPort> {
    repository: &'a R,
    pub snapshot: &'a mut Snapshot,
    completed: bool,
}

impl<'a, R: RepositoryPort> SnapshotGuard<'a, R> {
    pub fn new(repository: &'a R, snapshot: &'a mut Snapshot) -> Self {
        Self {
            repository,
            snapshot,
            completed: false,
        }
    }

    pub fn mark_completed(mut self) {
        self.completed = true;
    }
}

impl<'a, R: RepositoryPort> Drop for SnapshotGuard<'a, R> {
    fn drop(&mut self) {
        if !self.completed && self.snapshot.status == SnapshotStatus::Running {
            let _ = self.snapshot.interrupt();
            let _ = self.repository.update_snapshot(self.snapshot);
            tracing::warn!("⚠️ SnapshotGuard auto-cleaned interrupted snapshot {}", self.snapshot.id.0);
        }
    }
}

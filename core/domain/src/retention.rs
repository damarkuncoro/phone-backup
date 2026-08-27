use crate::{Snapshot, SnapshotId, SnapshotStatus};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetentionPolicy {
    pub keep_daily: u32,
    pub keep_weekly: u32,
    pub keep_monthly: u32,
}

impl Default for RetentionPolicy {
    fn default() -> Self {
        Self {
            keep_daily: 7,
            keep_weekly: 4,
            keep_monthly: 12,
        }
    }
}

pub trait RetentionStrategy {
    fn select_snapshots_to_delete(&self, snapshots: &[Snapshot]) -> Vec<SnapshotId>;
}

/// Retention strategy that keeps the N most recent completed snapshots and marks the rest for deletion.
#[derive(Debug, Clone)]
pub struct KeepCountStrategy {
    pub keep_limit: usize,
}

impl RetentionStrategy for KeepCountStrategy {
    fn select_snapshots_to_delete(&self, snapshots: &[Snapshot]) -> Vec<SnapshotId> {
        let mut completed: Vec<_> = snapshots
            .iter()
            .filter(|s| s.status == SnapshotStatus::Completed)
            .collect();

        completed.sort_by(|a, b| b.started_at.cmp(&a.started_at));

        if completed.len() > self.keep_limit {
            completed
                .into_iter()
                .skip(self.keep_limit)
                .map(|s| s.id.clone())
                .collect()
        } else {
            Vec::new()
        }
    }
}

/// Retention strategy that keeps 1 completed snapshot per day for the last N days.
#[derive(Debug, Clone)]
pub struct KeepDailyStrategy {
    pub keep_days: u32,
}

impl RetentionStrategy for KeepDailyStrategy {
    fn select_snapshots_to_delete(&self, snapshots: &[Snapshot]) -> Vec<SnapshotId> {
        use std::collections::HashSet;

        let mut completed: Vec<_> = snapshots
            .iter()
            .filter(|s| s.status == SnapshotStatus::Completed)
            .collect();

        completed.sort_by(|a, b| b.started_at.cmp(&a.started_at));

        let mut seen_days = HashSet::new();
        let mut to_delete = Vec::new();

        for s in completed {
            let day_key = s.started_at.format("%Y-%m-%d").to_string();
            if seen_days.len() >= self.keep_days as usize && !seen_days.contains(&day_key) {
                to_delete.push(s.id.clone());
            } else {
                seen_days.insert(day_key);
            }
        }

        to_delete
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::DeviceId;
    use chrono::Utc;

    #[test]
    fn test_keep_count_strategy() {
        let dev_id = DeviceId::new("DEV1");
        let mut snapshots = Vec::new();

        for i in 0..5 {
            let mut s = Snapshot::new(dev_id.clone());
            s.status = SnapshotStatus::Completed;
            s.started_at = Utc::now() - chrono::Duration::hours(i);
            snapshots.push(s);
        }

        let strategy = KeepCountStrategy { keep_limit: 2 };
        let to_delete = strategy.select_snapshots_to_delete(&snapshots);

        assert_eq!(to_delete.len(), 3);
    }
}

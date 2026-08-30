use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use crate::DeviceId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScheduleFrequency {
    Hourly,
    Daily,
    Weekly,
    OnConnect,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupSchedule {
    pub device_id: DeviceId,
    pub frequency: ScheduleFrequency,
    pub last_run_at: Option<DateTime<Utc>>,
    pub enabled: bool,
}

impl BackupSchedule {
    pub fn is_due(&self) -> bool {
        if !self.enabled {
            return false;
        }
        let now = Utc::now();
        match (self.last_run_at, self.frequency) {
            (None, _) => true,
            (Some(last), ScheduleFrequency::Hourly) => now.signed_duration_since(last).num_hours() >= 1,
            (Some(last), ScheduleFrequency::Daily) => now.signed_duration_since(last).num_days() >= 1,
            (Some(last), ScheduleFrequency::Weekly) => now.signed_duration_since(last).num_days() >= 7,
            (Some(_), ScheduleFrequency::OnConnect) => false,
        }
    }
}

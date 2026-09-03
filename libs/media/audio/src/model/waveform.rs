use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WaveformPeaks {
    pub points: Vec<u8>,
}

impl WaveformPeaks {
    pub fn new(points: Vec<u8>) -> Self {
        Self { points }
    }

    pub fn is_empty(&self) -> bool {
        self.points.is_empty()
    }

    pub fn count(&self) -> usize {
        self.points.len()
    }
}

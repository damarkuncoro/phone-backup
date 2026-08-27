use ports::ProgressObserver;

pub struct IndicatifProgressObserver;

impl IndicatifProgressObserver {
    pub fn new() -> Self {
        Self
    }
}

impl Default for IndicatifProgressObserver {
    fn default() -> Self {
        Self::new()
    }
}

impl ProgressObserver for IndicatifProgressObserver {
    fn start(&self, total_items: u64, _message: &str) {
        use indicatif::{ProgressBar, ProgressStyle};
        let pb = ProgressBar::new(total_items);
        let _ = pb.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta}) {msg}")
                .unwrap_or_else(|_| ProgressStyle::default_bar())
                .progress_chars("#>-"),
        );
    }

    fn update(&self, _current: u64, message: &str) {
        println!("{}", message);
    }

    fn finish(&self, message: &str) {
        println!("✨ {}", message);
    }
}

use ports::ProgressPort;
use indicatif::{ProgressBar, ProgressStyle};
use std::sync::Mutex;

pub struct CliProgress {
    pb: Mutex<Option<ProgressBar>>,
}

impl CliProgress {
    pub fn new() -> Self {
        Self {
            pb: Mutex::new(None),
        }
    }
}

impl ProgressPort for CliProgress {
    fn start(&self, total: u64, message: &str) {
        let pb = ProgressBar::new(total);
        pb.set_style(ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta}) {msg}")
            .unwrap()
            .progress_chars("#>-"));
        pb.set_message(message.to_string());
        let mut guard = self.pb.lock().unwrap();
        *guard = Some(pb);
    }

    fn inc(&self, amount: u64, message: &str) {
        let guard = self.pb.lock().unwrap();
        if let Some(pb) = guard.as_ref() {
            pb.inc(amount);
            pb.set_message(message.to_string());
        }
    }

    fn finish(&self, message: &str) {
        let mut guard = self.pb.lock().unwrap();
        if let Some(pb) = guard.take() {
            pb.finish_with_message(message.to_string());
        }
    }
}

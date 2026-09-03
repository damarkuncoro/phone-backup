use thiserror::Error;

#[derive(Debug, Clone)]
pub struct HoldingProcess {
    pub pid: u32,
    pub name: String,
    pub is_daemon: bool,
    pub launchd_label: Option<String>,
}

#[derive(Debug, Error)]
pub enum ResolverError {
    #[error("failed to run lsof: {0}")]
    Lsof(#[from] std::io::Error),
    #[error("failed to run launchctl: {0}")]
    Launchctl(std::io::Error),
    #[error("no conflicting process found via lsof")]
    NoHolderFound,
}

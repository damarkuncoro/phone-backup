use anyhow::Result;
use application::BackupService;

#[allow(dead_code)]
pub trait CliCommand<D, S, R, T, A, DP, P>
where
    D: ports::DevicePort,
    S: ports::ScannerPort,
    R: ports::RepositoryPort,
    T: ports::StoragePort,
    A: ports::AppProviderPort,
    DP: ports::DataProviderPort,
    P: ports::ProgressPort,
{
    fn execute(&self, service: &BackupService<D, S, R, T, A, DP, P>) -> Result<()>;
}

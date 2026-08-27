use anyhow::Result;
use application::BackupService;

pub trait CliCommand<D, S, R, T, A, DP>
where
    D: ports::DevicePort,
    S: ports::ScannerPort,
    R: ports::RepositoryPort,
    T: ports::StoragePort,
    A: ports::AppProviderPort,
    DP: ports::DataProviderPort,
{
    fn execute(&self, service: &BackupService<D, S, R, T, A, DP>) -> Result<()>;
}

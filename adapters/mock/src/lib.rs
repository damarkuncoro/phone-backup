pub mod app;
pub mod data;
pub mod device;
pub mod scanner;

pub use app::MockAppProvider;
pub use data::MockDataProvider;
pub use device::MockDeviceAdapter;
pub use scanner::MockScannerAdapter;

#[cfg(test)]
mod tests {
    use super::*;
    use domain::{Capability, DeviceId};
    use ports::DevicePort;

    #[test]
    fn discovers_the_seeded_device() {
        let adapter = MockDeviceAdapter::default();
        let devices = adapter.discover().unwrap();
        assert_eq!(devices.len(), 1);
        assert_eq!(devices[0].model, "Pixel 8");
    }

    #[test]
    fn unknown_device_info_errors() {
        let adapter = MockDeviceAdapter::default();
        let err = adapter.info(&DeviceId::new("NOPE"));
        assert!(err.is_err());
    }

    #[test]
    fn capability_matrix_flags_protected_data_as_denied() {
        let adapter = MockDeviceAdapter::default();
        let id = DeviceId::new("A1B2C3D4");
        let matrix = adapter.capabilities(&id).unwrap();
        assert!(matrix.is_available(Capability::ReadFiles));
        assert!(!matrix.is_available(Capability::ReadSms));
    }
}

#[derive(Debug)]
pub enum DeviceError {
    AccessDenied,
    DeviceOffline,
}

impl std::fmt::Display for DeviceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        use DeviceError::*;
        write!(
            f,
            "{}",
            match self {
                AccessDenied => "access is denied",
                DeviceOffline => "device is offline",
            }
        )
    }
}

impl std::error::Error for DeviceError {}

impl From<DeviceError> for String {
    fn from(e: DeviceError) -> Self {
        format!("{e}")
    }
}

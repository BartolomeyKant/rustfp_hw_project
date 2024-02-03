use super::*;

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

#[derive(Debug)]
pub struct Socket {
    pub description: String,
    pub is_on: bool,
    pub current_pover: f32,
    pub is_online: bool,
    pub is_accessable: bool,
}

impl Socket {
    pub fn description(&self) -> &str {
        self.description.as_str()
    }

    pub fn get_power(&self) -> Result<f32, DeviceError> {
        self.check_device()?;
        Ok(self.current_pover)
    }

    pub fn turn_off(&mut self) -> Result<(), DeviceError> {
        self.check_device()?;

        self.is_on = false;
        Ok(())
    }

    pub fn turn_on(&mut self) -> Result<(), DeviceError> {
        self.check_device()?;

        self.is_on = true;
        Ok(())
    }

    pub fn is_on(&self) -> Result<bool, DeviceError> {
        match self.check_device() {
            Err(e) => match e {
                DeviceError::DeviceOffline => Ok(false),
                _ => Err(e),
            },
            _ => Ok(self.is_on),
        }
    }

    fn check_device(&self) -> Result<(), DeviceError> {
        if !self.is_accessable {
            return Err(DeviceError::AccessDenied);
        }
        if !self.is_online {
            return Err(DeviceError::DeviceOffline);
        }
        Ok(())
    }
}

impl SmartDevice for Socket {
    fn state_report(&self) -> String {
        format!(
            "description: {}, current power: {}, state: {}",
            self.description(),
            match self.get_power() {
                Ok(power) => format!("{power}"),
                Err(err) => format!("{err}"),
            },
            match self.is_on() {
                Ok(on) =>
                    if on {
                        "is on now".to_string()
                    } else {
                        "is off now".to_string()
                    },
                Err(err) => format!("{err}"),
            }
        )
    }
}

#[derive(Debug)]
pub struct Thermometr {
    pub description: String,
    pub value: f32,
    pub is_online: bool,
}

impl Thermometr {
    pub fn description(&self) -> &str {
        self.description.as_str()
    }
    pub fn value(&self) -> Result<f32, DeviceError> {
        self.check_device()?;
        Ok(self.value)
    }

    fn check_device(&self) -> Result<(), DeviceError> {
        if !self.is_online {
            return Err(DeviceError::DeviceOffline);
        }
        Ok(())
    }
}

impl SmartDevice for Thermometr {
    fn state_report(&self) -> String {
        format!(
            "Thermometr: {} current value: {}",
            self.description(),
            match self.value() {
                Ok(val) => format!("{val}"),
                Err(error) => format!("{error}"),
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::smart_house::SmartDevice;

    use super::{Socket, Thermometr};

    #[test]
    fn socket_fields() {
        let mut socket = Socket {
            description: "socket 1".to_string(),
            is_on: false,
            current_pover: 12f32,
            is_accessable: true,
            is_online: true,
        };

        assert_eq!(socket.description(), "socket 1".to_string());
        assert_eq!(socket.get_power().unwrap(), 12f32);
        assert!(!socket.is_on().unwrap());
        let _ = socket.turn_on();
        assert!(socket.is_on().unwrap());
        let _ = socket.turn_off();
        assert!(!socket.is_on().unwrap());
    }

    #[test]
    #[should_panic]
    fn socket_errors_get_power() {
        let socket = Socket {
            description: "socket 1".to_string(),
            is_on: false,
            current_pover: 12f32,
            is_accessable: false,
            is_online: true,
        };

        socket.get_power().expect("power for not accessable");
    }

    #[test]
    #[should_panic]
    fn socket_errors_turn_on_off() {
        let mut socket = Socket {
            description: "socket 1".to_string(),
            is_on: false,
            current_pover: 12f32,
            is_accessable: false,
            is_online: true,
        };

        socket.turn_on().expect("turn on for not accessable");
        socket.turn_off().expect("turn off for not accessable");
    }

    #[test]
    fn thermometr_fields() {
        let thermometr = Thermometr {
            description: "thermometr 1".to_string(),
            value: 76f32,
            is_online: true,
        };

        assert_eq!(thermometr.description(), "thermometr 1".to_string());
        assert_eq!(thermometr.value().unwrap(), 76f32);
    }

    #[test]
    #[should_panic]
    fn thermometr_error() {
        let thermometr = Thermometr {
            description: "thermometr 1".to_string(),
            value: 76f32,
            is_online: false,
        };

        thermometr.value().expect("value for offline device");
    }

    fn assert_device_report<T: SmartDevice>(device: &T, report: &str) {
        assert_eq!(device.state_report().as_str(), report);
    }

    #[test]
    fn socket_report() {
        let socket = Socket {
            description: "socket 2".to_string(),
            is_on: true,
            current_pover: 122f32,
            is_accessable: true,
            is_online: true,
        };

        assert_device_report(
            &socket,
            "description: socket 2, current power: 122, state: is on now",
        );
    }

    #[test]
    fn thermometr_report() {
        let thermometr = Thermometr {
            description: "thermometr 2".to_string(),
            value: -235f32,
            is_online: true,
        };

        assert_device_report(&thermometr, "Thermometr: thermometr 2 current value: -235");
    }
}

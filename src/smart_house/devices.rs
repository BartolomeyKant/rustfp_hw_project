use super::*;

pub struct Socket {
    pub description: String,
    pub is_on: bool,
    pub current_pover: f32,
}

impl Socket {
    pub fn description(&self) -> &str {
        self.description.as_str()
    }

    pub fn get_power(&self) -> f32 {
        self.current_pover
    }

    pub fn turn_off(&mut self) {
        self.is_on = false;
    }

    pub fn turn_on(&mut self) {
        self.is_on = true;
    }
}

impl SmartDevice for Socket {
    fn state_report(&self) -> String {
        format!(
            "description: {}, current power: {}, state: {}",
            self.description(),
            self.get_power(),
            if self.is_on {
                "is on now"
            } else {
                "is off now"
            }
        )
    }
}

pub struct Thermometr {
    pub description: String,
    pub value: f32,
}

impl Thermometr {
    pub fn description(&self) -> &str {
        self.description.as_str()
    }
    pub fn value(&self) -> f32 {
        self.value
    }
}

impl SmartDevice for Thermometr {
    fn state_report(&self) -> String {
        format!(
            "Thermometr: {} current value: {}",
            self.description(),
            self.value()
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
        };

        assert_eq!(socket.description(), "socket 1".to_string());
        assert_eq!(socket.get_power(), 12f32);
        assert!(!socket.is_on);
        socket.turn_on();
        assert!(socket.is_on);
        socket.turn_off();
        assert!(!socket.is_on);
    }

    #[test]
    fn thermometr_fields() {
        let thermometr = Thermometr {
            description: "thermometr 1".to_string(),
            value: 76f32,
        };

        assert_eq!(thermometr.description(), "thermometr 1".to_string());
        assert_eq!(thermometr.value(), 76f32);
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
        };

        assert_device_report(&thermometr, "Thermometr: thermometr 2 current value: -235");
    }
}

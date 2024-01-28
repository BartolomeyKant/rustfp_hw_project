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
            "description: {}, current_power: {}, state: {}",
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
            "Thermometr: {} current value:{}",
            self.description(),
            self.value()
        )
    }
}

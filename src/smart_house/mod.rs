pub mod interface {
    pub use std::collections::HashMap;

    pub trait SmartDevice {
        fn state_report(&self) -> String;
    }

    pub trait SmartRoom {
        fn devices(&self) -> &HashMap<String, Box<dyn SmartDevice>>;
    }

    pub trait SmartHouse {
        fn name(&self) -> &str;
        fn rooms(&self) -> &HashMap<String, Box<dyn SmartRoom>>;
    }
}

pub use interface::*;

pub mod devices;

pub mod rooms;

pub mod house;

pub mod reports;

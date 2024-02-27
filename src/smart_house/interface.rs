pub trait Nameable {
    fn name(&self) -> String;
}

pub trait SmartDevice: Nameable {
    fn state_report(&self) -> String;
}

pub trait DeviceStorage {
    fn devices_in_room(&self, name: &str) -> Vec<&dyn SmartDevice>;
}

pub trait SmartRoom: Nameable {
    fn devices(&self) -> Vec<String>;
}

pub trait SmartHouse: Nameable {
    fn rooms(&self) -> Vec<&dyn SmartRoom>;
}

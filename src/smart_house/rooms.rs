use super::*;

pub struct SmartRoomImpl {
    devices: HashMap<String, Box<dyn SmartDevice>>,
}

impl SmartRoomImpl {
    pub fn new() -> SmartRoomImpl {
        SmartRoomImpl {
            devices: Default::default(),
        }
    }

    pub fn add_device<T: SmartDevice + 'static>(&mut self, name: &str, device: T) {
        self.devices.insert(name.to_string(), Box::new(device));
    }
}

impl SmartRoom for SmartRoomImpl {
    fn devices(&self) -> &HashMap<String, Box<dyn SmartDevice>> {
        &self.devices
    }
}

use std::cell::RefCell;
use std::rc::Rc;

use super::*;

pub struct SmartRoomImpl<TDevices: DeviceStorage> {
    name: String,
    devices: Rc<RefCell<TDevices>>,
}

impl<T: DeviceStorage> SmartRoomImpl<T> {
    pub fn new(name: &str, devices: Rc<RefCell<T>>) -> Self {
        SmartRoomImpl {
            name: name.to_string(),
            devices,
        }
    }
}

impl<T: DeviceStorage> Nameable for SmartRoomImpl<T> {
    fn name(&self) -> String {
        self.name.clone()
    }
}

impl<T: DeviceStorage> SmartRoom for SmartRoomImpl<T> {
    fn devices(&self) -> Vec<String> {
        let r = (*self.devices).borrow();
        r.devices_in_room(&self.name)
            .iter()
            .map(|d| d.name())
            .collect()
    }
}

use std::collections::HashMap;

use super::devices::*;
use crate::smart_house::interface::DeviceStorage;
use crate::smart_house::interface::SmartDevice;

pub enum DeviceType {
    Socket(Socket),
    Thermometr(Thermometr),
}

impl DeviceType {
    pub fn deref_to_smart_device(&self) -> &dyn SmartDevice {
        match self {
            DeviceType::Socket(d) => d,
            DeviceType::Thermometr(d) => d,
        }
    }
}

#[derive(PartialEq, Eq, Hash)]
struct DevicePosition {
    house: String,
    room: String,
    name: String,
}

pub struct DevicePositionQuery {
    pub house: String,
    pub room: String,
    pub name: String,
}

impl PartialEq<DevicePosition> for DevicePositionQuery {
    fn eq(&self, rhs: &DevicePosition) -> bool {
        let house = self.house == "*" || self.house == rhs.house;
        let room = self.room == "*" || self.room == rhs.room;
        let name = self.name == "*" || self.name == rhs.name;

        house && room && name
    }
}

pub struct DeviceStorageImpl {
    devices: HashMap<DevicePosition, DeviceType>,
}

impl DeviceStorageImpl {
    pub fn new() -> Self {
        Self {
            devices: Default::default(),
        }
    }

    pub fn add_device(&mut self, house: &str, room: &str, name: &str, device: DeviceType) {
        self.devices.insert(
            DevicePosition {
                house: house.to_string(),
                room: room.to_string(),
                name: name.to_string(),
            },
            device,
        );
    }

    pub fn remove_deivices(&mut self, query: &DevicePositionQuery) {
        self.devices.retain(|k, _| query != k);
    }

    pub fn query_devices(&self, query: &DevicePositionQuery) -> Vec<&DeviceType> {
        self.devices
            .iter()
            .filter(|d| query == d.0)
            .map(|d| d.1)
            .collect()
    }
}

impl DeviceStorage for DeviceStorageImpl {
    fn devices_in_room(&self, name: &str) -> Vec<&dyn SmartDevice> {
        let query_pos = DevicePositionQuery {
            house: "*".to_string(),
            room: name.to_string(),
            name: "*".to_string(),
        };

        self.query_devices(&query_pos)
            .iter()
            .map(|d| d.deref_to_smart_device())
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::smart_house::interface::Nameable;

    #[test]
    fn add_device() {
        let mut storage = DeviceStorageImpl::new();
        storage.add_device(
            "0",
            "1",
            "socket",
            DeviceType::Socket(Socket {
                name: "socket".to_string(),
                description: "socket".to_string(),
                is_on: false,
                current_pover: 12_f32,
                is_online: true,
                is_accessable: true,
            }),
        );
        storage.add_device(
            "0",
            "1",
            "socket2",
            DeviceType::Socket(Socket {
                name: "socket2".to_string(),
                description: "socket2".to_string(),
                is_on: false,
                current_pover: 12_f32,
                is_online: true,
                is_accessable: true,
            }),
        );

        // query device
        let dev = storage.query_devices(&DevicePositionQuery {
            house: "0".to_string(),
            room: "1".to_string(),
            name: "socket".to_string(),
        });
        assert_eq!(dev.len(), 1);

        match dev.first().unwrap()
        {
            DeviceType::Socket(s)=> assert_eq!(s.name(), "socket"),
            _ => panic!("wrong device"),
        }

        // device not exists
        let dev = storage.query_devices(&DevicePositionQuery {
            house: "1".to_string(),
            room: "1".to_string(),
            name: "socket".to_string(),
        });
        assert_eq!(dev.len(), 0);
    }

    #[test]
    fn remove_deivice() {
        let mut storage = DeviceStorageImpl::new();
        storage.add_device(
            "0",
            "1",
            "socket",
            DeviceType::Socket(Socket {
                name: "socket".to_string(),
                description: "socket".to_string(),
                is_on: false,
                current_pover: 12_f32,
                is_online: true,
                is_accessable: true,
            }),
        );

        let query = DevicePositionQuery {
            house: "0".to_string(),
            room: "1".to_string(),
            name: "socket".to_string(),
        };
        // query device
        let dev = storage.query_devices(&query);
        assert_eq!(dev.len(), 1);

        storage.remove_deivices(&query);

        // device not exists
        let dev = storage.query_devices(&query);
        assert_eq!(dev.len(), 0);
    }
}

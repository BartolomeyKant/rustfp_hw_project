use std::collections::HashMap;
use std::ffi::c_void;

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

pub struct Socket {
    description: String,
    is_on: bool,
    current_pover: f32,
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
    description: String,
    value: f32,
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

struct SmartRoomImpl {
    devices: HashMap<String, Box<dyn SmartDevice>>,
}

impl SmartRoomImpl {
    fn new() -> SmartRoomImpl {
        SmartRoomImpl {
            devices: Default::default(),
        }
    }

    fn add_device<T: SmartDevice + 'static>(&mut self, name: &str, device: T) {
        self.devices.insert(name.to_string(), Box::new(device));
    }
}

impl SmartRoom for SmartRoomImpl {
    fn devices(&self) -> &HashMap<String, Box<dyn SmartDevice>> {
        &self.devices
    }
}

struct SmartHouseImp {
    name: String,
    rooms: HashMap<String, Box<dyn SmartRoom>>,
}

impl SmartHouseImp {
    pub fn new(name: &str) -> SmartHouseImp {
        SmartHouseImp {
            name: name.to_string(),
            rooms: Default::default(),
        }
    }
    pub fn add_room<T: SmartRoom + 'static>(&mut self, name: &str, room: T) {
        self.rooms.insert(name.to_string(), Box::new(room));
    }
}

impl SmartHouse for SmartHouseImp {
    fn name(&self) -> &str {
        self.name.as_str()
    }
    fn rooms(&self) -> &HashMap<String, Box<dyn SmartRoom>> {
        &self.rooms
    }
}

pub fn house_revision_report(house: &dyn SmartHouse) -> Result<String, String> {
    Ok(format!("House name: {} {}", house.name(), {
        let mut rooms_report = String::new();
        for (room_name, room) in house.rooms() {
            let room_rep = format!("\n\troom: {} devices: {}", room_name, {
                let mut devices_list = String::new();
                for dev_name in room.devices().keys() {
                    let add_device = format!("{},", &dev_name);
                    devices_list = devices_list + &add_device;
                }
                devices_list
            });
            rooms_report = rooms_report + &room_rep;
        }
        rooms_report
    }))
}

pub fn make_device_report(
    house: &dyn SmartHouse,
    report_device: &dyn SmartDevice,
) -> Result<String, &'static str> {
    for (r_name, room) in house.rooms() {
        for (dev_name, dev) in room.devices() {
            let ptr1 = dev.as_ref() as *const _ as *const c_void;
            let ptr2 = report_device as *const _ as *const c_void;
            if std::ptr::eq(ptr1, ptr2) {
                return Ok(format!(
                    "Device {dev_name} placed in room {r_name}, device state is\n\t{}",
                    report_device.state_report()
                ));
            }
        }
    }
    Err("device not found!")
}

fn main() {
    let mut house = SmartHouseImp::new("my_house");
    for i in 0..3 {
        let mut room = SmartRoomImpl::new();
        room.add_device(
            "lamp socket",
            Socket {
                description: format!("Socket in {i} room"),
                is_on: true,
                current_pover: 12.0,
            },
        );
        room.add_device(
            "thermometr",
            Thermometr {
                description: format!("Thermometr in {i} room"),
                value: 34.0,
            },
        );
        house.add_room(format!("{i}_room").as_str(), room);
    }

    house.add_room("12th room", SmartRoomImpl::new());

    let house_revision = house_revision_report(&house).expect("Something went wrong");

    println!("House revision: {house_revision}");

    let room_socket_report = {
        match house.rooms().get("1_room") {
            Some(room) => match room.devices().get("lamp socket") {
                Some(device) => {
                    make_device_report(&house, &**device).expect("failed to get device report")
                }
                _ => panic!("device not found"),
            },
            _ => panic!("room not found"),
        }
    };

    println!("Device report\n{}", room_socket_report);
}

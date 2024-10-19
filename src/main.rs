use std::collections::HashMap;


trait  SmartDevice {
    fn description(&self) -> String;
}

struct Socket {
    description: String,
    state: bool,
    power: f32,
}

impl Socket {
    fn new(description: &str) -> Socket {
        Socket {
            description: String::from(description),
            state: false,
            power: 0.0,
        }
    }

    fn description(&self) -> String {
        format!(
            "{}, state: {}, power: {}",
            self.description, self.state, self.power
        )
    }
    fn toggle(&mut self) -> bool {
        self.state = !self.state;
        self.state
    }
    fn _power(&self) -> f32 {
        self.power
    }
}

impl SmartDevice for Socket {
    fn description(&self) -> String {
        self.description()
    }
}

struct Thermometer {
    temperature: f32,
}

impl Thermometer {
    fn new() -> Thermometer {
        Thermometer { temperature: 0.0 }
    }
    fn temperature(&self) -> f32 {
        self.temperature
    }
}

impl SmartDevice for Thermometer {
    fn description(&self) -> String {
        format!("Thermometer, temperature: {}", self.temperature())
    }
}


struct Home {
    name: String,
    rooms: Vec<Room>,
}

impl Home {
    fn new(name: &str) -> Home {
        Home {
            name: String::from(name),
            rooms: Vec::new(),
        }
    }

    fn rooms(&self) -> &Vec<Room> {
        &self.rooms
    }

    fn add_room(&mut self, room: Room) -> &mut Room {
        self.rooms.push(room);
        self.rooms.last_mut().unwrap()
    }
}

struct Room {
    name: String,
    devices: Vec<String>,
}

impl Room {
    fn new(name: & str) -> Room {
        Room{
            name: String::from(name),
            devices: Vec::new(),
        }
    }
    fn devices(&self) -> &Vec<String> {
        &self.devices
    }
    fn add_device(&mut self, device: &str) {
        self.devices.push(String::from(device));
    }
}


#[derive(Default)]
struct DeviceStorage {
    sockets: HashMap<String, Socket>,
    thermometers: HashMap<String, Thermometer>,
}

fn get_device_report(name: &str, dev_storage: &DeviceStorage) -> String {
    let device: &dyn SmartDevice = match dev_storage.sockets.get(name) {
        Some(socket) => socket,
        None => match dev_storage.thermometers.get(name) {
            Some(thermometer) => thermometer,
            None => panic!("Device not found"),
        },
    };

    device.description()
}

fn room_report(room: & Room, dev_storage: &DeviceStorage) -> String {
    let mut report = format!("Room: {}", room.name);
    for device in room.devices() {
        report.push_str(&format!("\n  {}", get_device_report(device, dev_storage)));
    }
    report
}

fn home_report(home:& Home, dev_storage: &DeviceStorage) -> String {
    let mut report = format!("Home: {}", home.name);
    for room in home.rooms() {
        report.push_str(&format!("\n{}", &room_report(room, dev_storage)));
    }
    report
}


fn main() {
    let mut dev_storage = DeviceStorage::default();

    let mut s1 = Socket::new("socket 1");
    s1.power= 10.0;
    s1.toggle();
    dev_storage.sockets.insert(String::from("socket1"), s1);
    dev_storage.sockets.insert(String::from("socket2"), Socket::new("socket 2"));
    dev_storage.sockets.insert(String::from("socket3"), Socket::new("socket 3"));
    dev_storage.sockets.insert(String::from("socket4"), Socket::new("socket 4"));

    let mut t1 = Thermometer::new();
    t1.temperature = 20.0;
    dev_storage.thermometers.insert(String::from("thermometer1"), t1);
    dev_storage.thermometers.insert(String::from("thermometer2"), Thermometer::new());
    dev_storage.thermometers.insert(String::from("thermometer3"), Thermometer::new());
    dev_storage.thermometers.insert(String::from("thermometer4"), Thermometer::new());


    let mut home = Home::new("My home");
    let room1 = home.add_room(Room::new("room1"));
    room1.add_device("socket1");
    room1.add_device("socket2");
    room1.add_device("thermometer1");

    let room2 = home.add_room(Room::new("room2"));
    room2.add_device("socket3");
    room2.add_device("socket4");
    room2.add_device("thermometer2");
    room2.add_device("thermometer3");
    room2.add_device("thermometer4");

    println!("{}", home_report(&home,  &dev_storage))
}

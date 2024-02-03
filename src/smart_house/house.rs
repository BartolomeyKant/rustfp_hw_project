use super::*;

pub struct SmartHouseImp {
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

use std::collections::HashMap;

use super::*;

pub struct SmartHouseImp {
    name: String,
    rooms: HashMap<String, Box<dyn SmartRoom>>,
}

impl SmartHouseImp {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            rooms: Default::default(),
        }
    }

    pub fn add_room<T: SmartRoom + 'static>(&mut self, name: &str, room: T) {
        self.rooms.insert(name.to_string(), Box::new(room));
    }
}

impl Nameable for SmartHouseImp {
    fn name(&self) -> String {
        self.name.clone()
    }
}

impl SmartHouse for SmartHouseImp {
    fn rooms(&self) -> Vec<&dyn SmartRoom> {
        self.rooms.iter().map(|r| r.1.as_ref()).collect()
    }
}

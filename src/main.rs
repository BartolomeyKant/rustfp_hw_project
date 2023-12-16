pub struct Plug {
    _id: String,
    _description: String,
    _is_on: bool,
    _current_pover: f32,
}

impl Plug {
    pub fn description(&self) -> String {
        todo!()
    }

    pub fn get_power(&self) -> f32 {
        todo!()
    }

    pub fn turn_off(&mut self) {
        todo!();
    }

    pub fn turn_on(&mut self) {
        todo!();
    }
}

pub struct Thermometr {
    _id: String,
    _value: f32,
}

impl Thermometr {
    pub fn value(&self) -> f32 {
        todo!()
    }
}

fn main() {}

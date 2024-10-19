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
    fn power(&self) -> f32 {
        self.power
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

fn main() {
    let mut socket = Socket::new("Socket in the living room");
    println!("Socket: {}", socket.description());
    println!("Socket: new state is {}", socket.toggle());
    println!("Socket: power is {}", socket.power());

    let thermometer = Thermometer::new();
    println!("Temperature: {}", thermometer.temperature());
}

use super::*;

pub fn house_revision_report(house: &dyn SmartHouse) -> Result<String, String> {
    Ok(format!("House name: {} {}", house.name(), {
        let mut rooms_report = String::new();
        for room in house.rooms() {
            let room_rep = format!("\n\troom: {} devices: {}", room.name(), {
                let mut devices_list = String::new();
                for dev in room.devices() {
                    let add_device = format!("{},", dev);
                    devices_list = devices_list + &add_device;
                }
                devices_list
            });
            rooms_report = rooms_report + &room_rep;
        }
        rooms_report
    }))
}

pub fn device_report(
    house: &dyn SmartHouse,
    report_device: &dyn SmartDevice,
) -> Result<String, &'static str> {
    for room in house.rooms() {
        for dev in room.devices() {
            if dev == report_device.name() {
                return Ok(format!(
                    "Device {} placed in room {}, device state is\n\t{}",
                    dev,
                    room.name(),
                    report_device.state_report()
                ));
            }
        }
    }
    Err("device not found!")
}

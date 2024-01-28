use super::*;

use std::ffi::c_void;

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

use std::cell::RefCell;
use std::rc::Rc;

use homeworks_project::smart_devices::device_storage::DevicePositionQuery;
use homeworks_project::smart_devices::device_storage::DeviceStorageImpl;
use homeworks_project::smart_devices::device_storage::DeviceType;
use homeworks_project::smart_devices::devices::Socket;
use homeworks_project::smart_devices::devices::Thermometr;
use homeworks_project::smart_house::house::SmartHouseImp;
use homeworks_project::smart_house::reports::device_report;
use homeworks_project::smart_house::reports::house_revision_report;
use homeworks_project::smart_house::rooms::SmartRoomImpl;
use homeworks_project::smart_house::Nameable;

fn main() {
    let device_storage = Rc::new(RefCell::new(DeviceStorageImpl::new()));

    let mut house = SmartHouseImp::new("my_house");

    for i in 0..3 {
        let room_name = format!("{i}_room");
        device_storage.borrow_mut().add_device(
            &house.name(),
            &room_name,
            "lamp socket",
            DeviceType::Socket(Socket {
                name: "lamp socket".to_string(),
                description: format!("Socket in {i} room"),
                is_on: true,
                current_pover: 12.0,
                is_accessable: true,
                is_online: false,
            }),
        );

        device_storage.borrow_mut().add_device(
            &house.name(),
            &room_name,
            "thermometr",
            DeviceType::Thermometr(Thermometr {
                name: "thermometr".to_string(),
                description: format!("Thermometr in {i} room"),
                value: 34.0,
                is_online: true,
            }),
        );
    }

    for i in 0..3 {
        let room = SmartRoomImpl::new(format!("{i}_room").as_str(), device_storage.clone());
        house.add_room(room.name().as_str(), room);
    }

    house.add_room(
        "12th room",
        SmartRoomImpl::new("12th room", device_storage.clone()),
    );

    let house_revision = house_revision_report(&house).expect("Something went wrong");

    println!("House revision: {house_revision}");

    let room_socket_report = {
        let dev_provider = device_storage.borrow();
        let devices = dev_provider.query_devices(&DevicePositionQuery {
            house: house.name(),
            room: "1_room".to_string(),
            name: "lamp socket".to_string(),
        });

        if devices.is_empty() {
            panic!("device not found");
        }
        let device = devices.first().unwrap();

        device_report(&house, device.deref_to_smart_device()).expect("failed to get device report")
    };

    println!("Device report\n{}", room_socket_report);

    house.remove_room("1_room");
    device_storage
        .borrow_mut()
        .remove_devices(&DevicePositionQuery {
            house: house.name(),
            room: "1_room".to_string(),
            name: "*".to_string(),
        });

    let house_revision = house_revision_report(&house).expect("Something went wrong");
    println!("House revision after remove: {house_revision}");
}

use homeworks_project::smart_house::devices::Socket;
use homeworks_project::smart_house::devices::Thermometr;
use homeworks_project::smart_house::house::SmartHouseImp;
use homeworks_project::smart_house::reports::house_revision_report;
use homeworks_project::smart_house::reports::make_device_report;
use homeworks_project::smart_house::rooms::SmartRoomImpl;
use homeworks_project::smart_house::SmartHouse;

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

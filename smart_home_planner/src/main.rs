//clone()  - useful for converting refs into values
use std::collections::HashMap;

struct SmartHome {
    name: String,
    rooms: HashMap<String, Room>,
}

impl SmartHome {
    fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            rooms: HashMap::new(),
        }
    }

    fn get_room_list(&self) -> &HashMap<String, Room> {
        &self.rooms
    }

    fn add_room(&mut self, new_room: &Room) {
        self.rooms.insert(new_room.name.clone(), new_room.clone());
    }

    fn remove_room(&mut self, room_name: &str) {
        self.rooms.remove(room_name.clone());
    }

    fn get_full_report<T: DeviceStorage>(&self, query: &T) {
        //iterate over all the rooms running through the devices located inside
        let room_list = &self.rooms;

        for (room_name, room) in room_list.iter() {
            println!("Entering {}", room_name);

            let mut device_list = &room.smart_devices;

            if device_list.is_empty() {
                println!("No smart devices in the room {}", room_name);
            }

            for (device_name, device) in device_list.iter() {
                let temporary_result = query.seek(room_name, device.clone());
                if let Some(available_device_type) = temporary_result {
                    available_device_type.show_description();
                } else {
                    println!("Sorry! Unable to define the current device state due to sanctions.");
                }
            }
        }
    }
}

#[derive(Clone)]
struct Room {
    name: String,
    smart_devices: HashMap<String, SmartDevice>,
}

impl Room {
    fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            smart_devices: HashMap::new(),
        }
    }

    fn get_device_list(&self) -> Vec<String> {
        let devices_to_show = &self.smart_devices;
        let mut device_list = Vec::new();

        for (device_name, _smart_device) in devices_to_show.iter() {
            device_list.push(String::from(device_name));
        }

        device_list
    }

    fn add_device(&mut self, new_device: &SmartDevice) {
        self.smart_devices
            .insert(new_device.name.clone(), new_device.clone());
    }

    fn remove_device(&mut self, id: &str) {
        self.smart_devices.remove(id.clone());
    }
}

#[derive(Clone, Eq, Hash, PartialEq)]
struct SmartDevice {
    name: String,
}

impl SmartDevice {
    fn new(name: &str) -> Self {
        Self { name: name.into() }
    }
}

trait DeviceStorage {
    fn seek(&self, room_name: &str, device: SmartDevice) -> Option<&dyn CanShowDescription>;
}

trait CanShowDescription {
    fn show_description(&self);
}

fn main() {
    impl DeviceStorage for HashMap<(String, SmartDevice), AvailableDevicesTypes> {
        fn seek(&self, room_name: &str, device: SmartDevice) -> Option<&dyn CanShowDescription> {
            self.get(&(room_name.into(), device))
                .map(|device| device as &dyn CanShowDescription)
        }
    }
    struct SmartOutlet {
        description: String,
        enabled: bool,
        consumption: f32,
    }

    impl SmartOutlet {
        fn _show_description(&self) {
            todo!()
        }

        fn _turn_on(&mut self) {
            todo!()
        }

        fn _turn_off(&mut self) {
            todo!()
        }

        fn _get_current_power_consumption(&self) -> f32 {
            todo!()
        }
    }
    struct SmartThermometer {
        current_temperature: f32,
    }

    impl SmartThermometer {
        fn _get_current_value(&self) -> f32 {
            todo!()
        }
    }

    enum AvailableDevicesTypes {
        SmartThermometer(SmartThermometer),
        SmartOutlet(SmartOutlet),
    }

    impl CanShowDescription for AvailableDevicesTypes {
        fn show_description(&self) {
            match self {
                AvailableDevicesTypes::SmartOutlet(SmartOutlet) => println!(
                    "SmartOutlet_{} : active: {}, consumption {}",
                    SmartOutlet.description, SmartOutlet.enabled, SmartOutlet.consumption
                ),
                AvailableDevicesTypes::SmartThermometer(SmartThermometer) => println!(
                    "SmartThermometer: current temperature: {}",
                    SmartThermometer.current_temperature
                ),
            }
        }
    }

    let mut home = SmartHome::new("Brand new home");

    let white_smart_outlet = SmartOutlet {
        description: String::from("Schneider"),
        enabled: true,
        consumption: 25.5,
    };
    let black_smart_outlet = SmartOutlet {
        description: String::from("Schneider"),
        enabled: false,
        consumption: 100.0,
    };
    let smart_thermometer = SmartThermometer {
        current_temperature: 24.3,
    };

    let mut living_room = Room::new("Living room");
    let mut kitchen = Room::new("Kitchen");
    let mut hall = Room::new("Hall");

    let mut device_types_available: HashMap<(String, SmartDevice), AvailableDevicesTypes> =
        HashMap::new();
    let smart_outlet_from_living_room = ("Living room".into(), SmartDevice::new("White outlet"));
    let smart_thermo_from_living_room =
        ("Living room".into(), SmartDevice::new("Omron thermometer"));

    let smart_outlet_from_kitchen = ("Kitchen".into(), SmartDevice::new("Black outlet"));

    living_room.add_device(&smart_outlet_from_living_room.1);
    living_room.add_device(&smart_thermo_from_living_room.1);

    kitchen.add_device(&smart_outlet_from_kitchen.1);

    home.add_room(&living_room);
    home.add_room(&kitchen);
    home.add_room(&hall);

    //synthetic relation between rooms and devices for full_device_report method
    device_types_available.insert(
        smart_outlet_from_living_room,
        AvailableDevicesTypes::SmartOutlet(white_smart_outlet),
    );
    device_types_available.insert(
        smart_thermo_from_living_room,
        AvailableDevicesTypes::SmartThermometer(smart_thermometer),
    );
    device_types_available.insert(
        smart_outlet_from_kitchen,
        AvailableDevicesTypes::SmartOutlet(black_smart_outlet),
    );

    home.get_full_report(&device_types_available);
}

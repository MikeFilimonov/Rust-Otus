use backend_framework_in_action::{DeviceStorage, Room, ShowDescription, SmartDevice, SmartHome};
use std::collections::HashMap;

fn main() {
    struct LocalStorage(HashMap<(String, SmartDevice), AvailableDeviceTypes>);

    enum AvailableDeviceTypes {
        SmartThermometer(SmartThermometer),
        SmartOutlet(SmartOutlet),
    }

    impl DeviceStorage for LocalStorage {
        fn seek(&self, room_name: &str, device: SmartDevice) -> Option<&dyn ShowDescription> {
            self.0
                .get(&(room_name.into(), device))
                .map(|device| device as &dyn ShowDescription)
        }
    }

    impl ShowDescription for AvailableDeviceTypes {
        fn show_description(&self) {
            match self {
                AvailableDeviceTypes::SmartOutlet(smart_outlet) => println!(
                    "SmartOutlet_{} : active: {}, consumption: {} W",
                    smart_outlet.description, smart_outlet.enabled, smart_outlet.consumption
                ),
                AvailableDeviceTypes::SmartThermometer(smart_thermometer) => println!(
                    "SmartThermometer: current temperature: {} C",
                    smart_thermometer.current_temperature
                ),
            }
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

    let mut home = SmartHome::new("Brand new smart home");

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
    let hall = Room::new("Hall");
    let bathroom = Room::new("Bathroom");

    let mut device_types_available: LocalStorage = LocalStorage(HashMap::new());
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
    home.add_room(&bathroom);

    let room_name = &bathroom.get_room_name();
    //changed mind to adding a bath - that's datcha
    home.remove_room(room_name);

    //synthetic relation between rooms and devices for full_device_report method
    device_types_available.0.insert(
        smart_outlet_from_living_room,
        AvailableDeviceTypes::SmartOutlet(white_smart_outlet),
    );
    device_types_available.0.insert(
        smart_thermo_from_living_room,
        AvailableDeviceTypes::SmartThermometer(smart_thermometer),
    );
    device_types_available.0.insert(
        smart_outlet_from_kitchen,
        AvailableDeviceTypes::SmartOutlet(black_smart_outlet),
    );

    home.get_full_report(&device_types_available);

    

}

#[cfg(test)]
mod tests {}

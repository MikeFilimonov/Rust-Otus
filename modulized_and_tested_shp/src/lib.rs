use std::collections::{HashMap, HashSet};

pub struct SmartHome {
    name: String,
    rooms: HashMap<String, Room>,
}

pub trait ShowDescription {
    fn show_description(&self);
}
pub trait DeviceStorage {
    fn seek(&self, room_name: &str, device: SmartDevice) -> Option<&dyn ShowDescription>;
}

impl SmartHome {

pub  fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            rooms: HashMap::new(),
        }
    }

pub fn _get_room_list(&self) -> &HashMap<String, Room> {
        &self.rooms
    }

pub fn add_room(&mut self, new_room: &Room) {
        self.rooms.insert(new_room.name.clone(), new_room.clone());
    }

pub fn remove_room(&mut self, room_name: &str) {
        self.rooms.remove(room_name);
    }

pub fn get_full_report<T: DeviceStorage>(&self, query: &T) {
        //iterate over all the rooms running through the devices located inside
        let room_list = &self.rooms;

        println!("Opening the door of the {}", self.name);

        for (room_name, room) in room_list.iter() {
            println!("Entering {}", room_name);

            let device_list = &room.smart_devices;

            if device_list.is_empty() {
                println!("No smart devices in the room {}", room_name);
            }

                for device in device_list.iter() {
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
pub struct Room {
    name: String,
    smart_devices: HashSet<SmartDevice>,
}

impl Room {
pub  fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            smart_devices: HashSet::new(),
        }
    }

pub fn get_room_name(&self) -> String {
    String::from(&self.name)
}

pub   fn _get_device_list(&self) -> Vec<&SmartDevice> {
        let devices_to_show = &self.smart_devices;
        let mut device_list = Vec::new();

        for device in devices_to_show.iter() {
            device_list.push(device);
        }

        device_list
    }

pub   fn add_device(&mut self, new_device: &SmartDevice) {
        self.smart_devices.insert(new_device.clone());
    }

pub   fn _remove_device(&mut self, device: &SmartDevice) {
        // self.smart_devices.remove(device.into());
        self.smart_devices.remove(device);
    }
}

#[derive(Clone, Eq, Hash, PartialEq)]
pub struct SmartDevice {
    name: String,
}

impl SmartDevice {
    pub  fn new(name: &str) -> Self {
        Self { name: name.into() }
    }
}


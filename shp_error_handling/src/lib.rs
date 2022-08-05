use std::collections::{HashMap, HashSet};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RoomPlannerError {
    #[error("The device already exists")]
    DeviceAlreadyExists,

    #[error("The device is not found")]
    DeviceNotFound,
}

#[derive(Error, Debug)]
pub enum SmartHomePlannerError {
    #[error("Please, define the room name and try again...")]
    InexistentRoom,
}

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Sorry, the room can't return the device list properly")]
    MainFeatureFailed,
}

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
    pub fn new(name: &str) -> Self {
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

    pub fn remove_room(&mut self, room_name: Option<&str>) -> Result<(), SmartHomePlannerError> {
        match room_name {
            None => return Err(SmartHomePlannerError::InexistentRoom),
            Some(room_name) => {
                let room_to_be_removed = room_name;
                self.rooms.remove(room_to_be_removed)
            }
        };
        Ok(())
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Room {
    name: String,
    smart_devices: HashSet<SmartDevice>,
}

impl Room {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            smart_devices: HashSet::new(),
        }
    }

    pub fn get_room_name(&self) -> String {
        String::from(&self.name)
    }

    pub fn _get_device_list(&self) -> Vec<&SmartDevice> {
        let devices_to_show = &self.smart_devices;
        let mut device_list = Vec::new();

        for device in devices_to_show.iter() {
            device_list.push(device);
        }

        device_list
    }

    pub fn add_device(&mut self, new_device: &SmartDevice) -> Result<(), RoomPlannerError> {
        let successfully_added = self.smart_devices.insert(new_device.clone());

        match successfully_added {
            true => Ok(()),
            _ => Err(RoomPlannerError::DeviceAlreadyExists),
        }
    }

    pub fn _remove_device(&mut self, device: &SmartDevice) -> Result<(), RoomPlannerError> {
        let removed = self.smart_devices.remove(device);
        match removed {
            true => Ok(()),
            _ => Err(RoomPlannerError::DeviceNotFound),
        }
    }
}

#[derive(Clone, Eq, Hash, PartialEq, Debug)]
pub struct SmartDevice {
    name: String,
}

impl SmartDevice {
    pub fn new(name: &str) -> Self {
        Self { name: name.into() }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn smart_home_supports_adding_rooms() {
        let mut depot = SmartHome::new("Storage facilities");
        let new_room_name = String::from("warehouse");
        let main_space = Room::new(&new_room_name);
        depot.add_room(&main_space);
        let current_room_list = depot._get_room_list();

        assert_eq!(&current_room_list.get(&new_room_name), &Some(&main_space));

        let absent_room_name = String::from("cabinet");
        assert_eq!(&current_room_list.get(&absent_room_name), &None);
    }

    #[test]
    fn smart_home_supports_removing_rooms() {
        let mut depot = SmartHome::new("Storage facilities");

        let warehouse_name = String::from("warehouse");
        let guards_room_name = String::from("security post");

        let warehouse = Room::new(&warehouse_name);
        let security_post = Room::new(&guards_room_name);
        depot.add_room(&warehouse);
        depot.add_room(&security_post);

        depot
            .remove_room(Some(&warehouse_name))
            .unwrap_or_else(|err| println!("{:?}", err));

        let current_room_list = depot._get_room_list();

        assert_eq!(&current_room_list.get(&warehouse_name), &None);
        assert_ne!(&current_room_list.get(&warehouse_name), &Some(&warehouse));

        let absent_room_name = String::from("cabinet");
        assert_eq!(&current_room_list.get(&absent_room_name), &None);

        assert_eq!(
            &current_room_list.get(&guards_room_name),
            &Some(&security_post)
        );
    }

    #[test]
    fn room_can_return_device_list() -> Result<(), AppError> {
        let smart_socket = SmartDevice::new("smart socket");
        let smart_bin = SmartDevice::new("smart trashbin");
        let smart_frige = SmartDevice::new("Samsung");

        let mut kitchen = Room::new("kitchen");
        kitchen
            .add_device(&smart_socket)
            .unwrap_or_else(|err| println!("{:?}", err));
        kitchen
            .add_device(&smart_bin)
            .unwrap_or_else(|err| println!("{:?}", err));
        kitchen
            .add_device(&smart_frige)
            .unwrap_or_else(|err| println!("{:?}", err));

        let devices_in_the_room = kitchen._get_device_list();

        let it_works = devices_in_the_room.len() > 0
            && devices_in_the_room.contains(&&smart_socket)
            && devices_in_the_room.contains(&&smart_bin)
            && devices_in_the_room.contains(&&smart_frige);

        if it_works {
            Ok(())
        } else {
            Err(AppError::MainFeatureFailed)
        }
    }
}

use std::collections::HashMap;

struct SmartHome{
    
    name : String,
    rooms: HashMap<String, Room>
}

impl SmartHome {
    
    fn get_room_list (&self) -> &HashMap<String, Room>{
        &self.rooms
    }

    fn add_room (&mut self, room_name: &str){
        
        // self.rooms.entry(room_name).or_insert(String::from(room_name), Room::new(&room_name));  
        self.rooms.insert(String::from(room_name), Room::new(&room_name)); 
    }

    fn remove_room (&mut self, room_name: &str){
        
        /*match self.rooms.get(room_name){
            Some(value) => self.rooms.remove(&room_name),
            None => println!("The room named {} doesn't exist", &room_name)
        }*/
        self.rooms.remove(&room_name);
    }

}

struct Room {
    name: String,
    smart_devices: HashMap<String, SmartDevice> 
}

impl Room {
    
    fn new (value: &str) -> Self {

        Self.name,
        smart_devices: HashMap<&str, SmartDevice>;
    }

    fn get_device_list (&self) -> &HashMap<String, SmartDevice> {
        &self.smart_devices
    }
    
    fn add_device (&mut self, id: &str){
        // self.smart_devices.entry(id).or_insert(String::from(id), SmartDevice::new(&id))
        self.smart_devices.insert(String::from(id), SmartDevice::new(&id));       
    }

    fn remove_device (&mut self, id: &str){
        
        /*match self.smart_devices.get(id){
            Some(value) => self.smart_devices.remove(&id),
            None => println!("The device named {} doesn't exist", &id)
        }*/
        self.smart_devices.remove(&id);
    }

}

struct SmartDevice {
    name: String
}

impl SmartDevice {
    fn new(value: &str)-> Box<dyn Device>{
       
        /*self.name = value;
        Self*/
        Box::new(value)

    }
}

trait Device {
    fn new (&self, name: String) -> Self{
        Self{name}
    }

}

fn main () {
    println!("Hello, world!");
}

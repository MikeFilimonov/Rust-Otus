use std::collections::HashMap;

struct SmartHome{
    
    name : &str,
    rooms: HashMap<&str, room>::new()
}

impl SmartHome {
    
    fn get_room_list (&self) -> HashMap<&str, room>{
        self.rooms
    }

    fn add_room (&mut self, room_name: &str){
        self.rooms.entry(room_name).or_insert(String::from(room_name), room::new(&room_name));   
    }

    fn remove_room(&mut self, room_name: &str){
        
        match self.rooms.get(room_name){
            Some(value) => self.rooms.remove(&room_name),
            None => println!("The room named {} doesn't exist", &room_name)
        }
    }

}

struct Room {
    name: String,
    smart_devices: HashMap<&str, smart_device>::new() 
}

impl Room {
    
    fn new(value: &str) -> Self {
        
        Room {
            name,
            smart_devices: HashMap<&str, smart_device>::new(),
            }
    }


}





struct SmartDevice {
    name: &str
    
}

impl SmartDevice {
    fn new(value: &str)->Self{
        Self{value}
    }
}



fn main() {
    println!("Hello, world!");
}

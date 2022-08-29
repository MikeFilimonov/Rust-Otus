// use backend_framework_in_action::*;

// #[test]
// fn smart_home_supports_adding_rooms() {
//     let mut depot = SmartHome::new("Storage facilities");
//     let new_room_name = String::from("warehouse");
//     let main_space = Room::new(&new_room_name);
//     depot.add_room(&main_space);
//     let current_room_list = depot._get_room_list();

//     assert_eq!(&current_room_list.get(&new_room_name), &Some(&main_space));

//     let absent_room_name = String::from("cabinet");
//     assert_eq!(&current_room_list.get(&absent_room_name), &None);
// }

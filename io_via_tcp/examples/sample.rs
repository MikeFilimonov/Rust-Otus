use io_via_tcp::{
    consts::{DEFAULT_OUTLET_ADDRESS},
    Command, SmartOutletClient,
};
use std::io;

fn main() {
    let default_ip = DEFAULT_OUTLET_ADDRESS;
    let mut client = SmartOutletClient::new(default_ip).unwrap();

    loop {
        show_disclaimer();

        let user_input = handle_user_input();

        let response = match user_input {
            Some(command) => {client.execute(command).unwrap();},
            None => {
                println!("No user input detected...");
                break;
            }
        };

        println!("Response: {:?}", response);
       

    }
}

fn show_disclaimer() {
    let prompt = "Available commands:\n\
    0: check consumption\n\
    1: check state \n\
    2: toggle outlet state \n\
    _: locked feature";

    print!("{}", prompt);
}

fn handle_user_input() -> Option<Command> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let command = match input.trim() {
        "0" => Command::CheckConsumption,
        "1" => Command::CheckState,
        "2" => Command::ToggleState,
        _ => return None,
    };

    Some(command)
}

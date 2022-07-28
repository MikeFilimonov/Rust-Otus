use async_tcp_server::{consts::DEFAULT_OUTLET_ADDRESS, Command, SmartOutletClient};
use std::io;

#[tokio::main]
async fn main() {
    let default_ip = DEFAULT_OUTLET_ADDRESS;
    let mut client = SmartOutletClient::new(default_ip).await.unwrap();

    loop {
        show_disclaimer();

        let user_input = handle_user_input();

        let response = match user_input {
            Some(command) => client.execute(command).await.unwrap(),
            None => {
                println!("Undefined command.Terminating the client app...");
                break;
            }
        };

        println!("[so]: {response}");
    }
}

fn show_disclaimer() {
    let prompt = "\n\
    Accessed the smart outlet. Now you can:\n\
    0: check its consumption\n\
    1: check its state \n\
    2: toggle the outlet state \n\
    _: quit";

    println!("{}", prompt);
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

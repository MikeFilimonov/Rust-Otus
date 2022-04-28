fn main() {
    
   let mut state = State::Menu;

   loop{
       
        let new_state:State = state.update();
       
        if let State::Exit = new_state{
           break;
       }

       state = new_state;
   }

}

enum State{
    
    Menu,
    Guessing(u32),
    Exit
}

impl State {
    
    pub fn update(&self) -> Self{

       match self{
           
           Self::Menu => Self::run_menu(),
           Self::Guessing(number) => Self::run_guessing(*number),
           Self::Exit => panic!("try to run exit state"),
       }

    }

fn run_menu() -> Self {
   
    println!();
    println!("**** MENU ****");
    println!("1) Guessing");
    println!("Other) Exit");
    println!("Your choice: ");

    let choice: Option<u32> = Self::read_input();
   
    match choice{
        Some(1) => Self::Guessing(Self::random_number()),
        _ => Self::Exit,
    }
}

fn read_input() -> Option<u32> {
    
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let number = buffer.trim().parse().ok()?;
    Some(number)

}

fn random_number() -> u32{
    let random_number: u32 = rand::random();
    (random_number % 100) as _
}

fn run_guessing(number: u32) -> Self {
    
    println!();
    println!("**** GUESSING ****");
    println!("Number from 0 to 99 to guess");
    println!("Other for go to Menu");
    println!("Your choice:");
    loop {

        let choice = Self::read_input();
        match choice {

            Some(x) if x < 100 &&  x < number => println!("Too small!"),
            Some(x) if x < 100 &&  x > number => println!("Too big!"),
            Some(x) if x < 100 &&  x == number => {
                
                println!("Right! You won!");
                break;
            }
            _ => break,

        } 
        
        }

        Self::Menu
    }
}

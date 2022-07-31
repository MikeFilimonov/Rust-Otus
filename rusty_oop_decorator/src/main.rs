pub struct Message {
    text: String,
}

pub trait Printable {
    fn print(&self) -> String;
}

impl Printable for Message {
    fn print(&self) -> String {
        self.text.clone()
    }
}

pub struct UnderlinedMessage<T> {
    underlined: T,
}

impl<T: Printable> Printable for UnderlinedMessage<T> {
    fn print(&self) -> String {
 
        let mut result = self.underlined.print();
        let length = result.len();
        result.push_str(&"_".repeat(length));
        result
        
    }
}

pub struct IndentedMessage<T> {
    indented: T,
}

impl<T: Printable> Printable for IndentedMessage<T> {
    fn print(&self) -> String {
        let result = "     ".to_string() + &self.indented.print() + "     ";
        result.replace('\n', "\n     ")
    }
}

fn main() {
    let text = Message {
        text: "Hi there! I gonna be followed by a long underscore..".to_string(),
    };
    let underlined = UnderlinedMessage { underlined: text };
    println!("Underlined result: {}", underlined.print());

    let text = Message {
        text: "Here we go to get indented".to_string(),
    };
    let indented = IndentedMessage { indented: text };
    println!("Indented result: {} ", indented.print());

    let text = Message {
        text: "Indention before an extra underscore".to_string(),
    };
    let indented = IndentedMessage { indented: text };
    let underlined = UnderlinedMessage {
        underlined: indented,
    };
    println!("Indention before Underlining {}", underlined.print());
}

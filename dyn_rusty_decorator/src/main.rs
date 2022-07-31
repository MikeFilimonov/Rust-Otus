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

pub struct UnderlinedMessage {
    underlined: Box<dyn Printable>,
}

impl UnderlinedMessage {
    pub fn new(underlined: Box<dyn Printable>) -> Self {
        Self { underlined }
    }
}

impl Printable for UnderlinedMessage {
    fn print(&self) -> String {
        let mut result = self.underlined.print();
        let length = result.len();
        result.push_str(&"_".repeat(length));
        result
    }
}

pub struct IndentedMessage {
    indented: Box<dyn Printable>,
}

impl IndentedMessage {
    pub fn new(indented: Box<dyn Printable>) -> Self {
        Self { indented }
    }
}

impl Printable for IndentedMessage {
    fn print(&self) -> String {
        let result = "     ".to_string() + &self.indented.print() + "     ";
        result.replace('\n', "\n     ")
    }
}

fn main() {
    let text = Message {
        text: "Hi there! I gonna be followed by a long underscore..".to_string(),
    };
    let underlined = UnderlinedMessage::new(Box::new(text));
    println!("Underlined result: {}", underlined.print());

    let text = Message {
        text: "Here we go to get indented".to_string(),
    };
    let indented = IndentedMessage::new(Box::new(text));
    println!("Indented result: {} ", indented.print());

    let text = Message {
        text: "Indention before an extra underscore".to_string(),
    };
    let indented = IndentedMessage::new(Box::new(text));
    let underlined = UnderlinedMessage::new(Box::new(indented));

    println!("Indention before Underlining {}", underlined.print());
}

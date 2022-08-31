use iced::{
    alignment, button, executor, time, Alignment, Application, Button, Column, Command, Container,
    Element, Length, Row, Settings, Subscription, Text,
};
use rand::thread_rng;

mod styles;

pub fn main() -> iced::Result {
    SmartOutlet::run(Settings::default())
}

struct SmartOutlet {
    state: State,
    voltage: i16,
    summary: String,
    toggle: button::State,
}

#[derive(Debug)]
enum State {
    On,
    Off,
}

#[derive(Debug, Clone)]
enum Message {
    Toggle,
}

impl Application for SmartOutlet {
    type Executor = executor::Default();
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            SmartOutlet {
                state: State::Off,
                voltage: i16::default(),
                summary: format!(
                    "The socket is: {}, {}V",
                    State::Off,
                    thread_rng().gen_range(0..2)
                ),
                toggle: button::State::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Hail to Rust")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Toggle => match self.state {
                State::Off => {
                    self.state = State::On;
                    self.voltage = thread_rng().gen_range(220..=230);
                }
                State::On => {
                    self.state = State::Off;
                    self.voltage = thread_rng().gen_range(0..2);
                }
            },
        }
        self.summary = format!("The socket is: {}, {}V", self.state, self.voltage);

        Command::none()
    }

    fn view(&mut self) -> Element<Message> {
        let display_view = Text::new(self.summary).size(50);

        let button = |state, label, style| {
            Button::new(
                state,
                Text::new(label).horizontal_alignment(alignment::Horizontal::Center),
            )
            .padding(20)
            .width(Length::Units(58))
            .style(style)
        };

        let toggle_button = {
            let (label, colour) = match self.state {
                State::Off => ("Turn on", styles::Button::Primary),
                State::On => ("Turn off", styles::Button::Destructive),
            };
        };

        let controls = Row::new().spacing(20).push(toggle_button);

        let content = Column::new()
            .align_items(Alignment::Center)
            .spacing(20)
            .push(button)
            .push(controls);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

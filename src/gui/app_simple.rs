use iced::widget::{button, column, text};
use iced::{Application, Command, Element, Theme};

#[derive(Debug, Clone)]
pub enum Message {
    IncrementPressed,
}

pub struct Counter {
    value: i32,
}

impl Application for Counter {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (Self { value: 0 }, Command::none())
    }

    fn title(&self) -> String {
        "Simple Counter".to_string()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::IncrementPressed => {
                self.value += 1;
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message> {
        column![
            text(format!("Value: {}", self.value)),
            button("Increment").on_press(Message::IncrementPressed)
        ]
        .into()
    }
}
// Test file to check iced 0.13 API
use iced::widget::{button, column, text};
use iced::{executor, Alignment, Element, Settings, Theme};

#[derive(Debug, Clone)]
enum Message {
    Test,
}

struct TestApp;

// Let's see what traits and types are available
impl iced::Application for TestApp {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Task<Self::Message>) {
        (Self, iced::Task::none())
    }

    fn title(&self) -> String {
        "Test".to_string()
    }

    fn update(&mut self, _message: Self::Message) -> iced::Task<Self::Message> {
        iced::Task::none()
    }

    fn view(&self) -> Element<'_, Self::Message> {
        column![text("Test")].into()
    }
}
extern crate env_logger;

use log::info;
use iced::{Align, Checkbox, Column, Element, Sandbox, Settings, Length};
use super::system;

pub fn main() -> iced::Result {
    env_logger::init();
    info!("Starting GUI for NoSleep.");
    NoSleep::run(Settings {
            window: iced::window::Settings {
                size: (200, 80),
                resizable: false,
                ..iced::window::Settings::default()
            },
            ..Settings::default()
        }
    )
}

#[derive(Default)]
struct NoSleep {
    activated: bool,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Toggled(bool),
}

impl Sandbox for NoSleep {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("NoSleep")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Toggled(value) => {
                self.activated = value;
                if self.activated {
                    system::prevent_sleep();
                } else {
                    system::default_sleep();
                }
            },
        }
    }

    fn view(&mut self) -> Element<Message> {
        Column::new()
            .padding(20)
            .align_items(Align::Center)
            .push(
                Checkbox::new(
                    self.activated,
                    "Prevent Sleep",
                    Message::Toggled
                )
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

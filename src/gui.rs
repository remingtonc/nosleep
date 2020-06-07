use iced::{Align, Checkbox, Column, Element, Sandbox, Settings};
use ::nosleep;

pub fn main() {
    NoSleep::run(Settings::default())
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
                    nosleep::prevent_sleep();
                } else {
                    nosleep::default_sleep();
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
            .into()
    }
}

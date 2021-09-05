//! Impulse Rocket Simulation Tool

#![deny(unsafe_code)]

use iced::{button, window, Align, Button, Column, Element, Sandbox, Settings, Text};

fn main() -> iced::Result {
    Counter::run(Settings {
        window: window::Settings {
            size: (100, 100),
            ..Default::default()
        },
        antialiasing: true,
        ..Default::default()
    })
}

#[derive(Default, Debug)]
struct Counter {
    value: i32,

    increment_button: button::State,
    decrement_button: button::State,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Increment,
    Decrement,
}

impl Sandbox for Counter {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Counter Test")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::Increment => self.value += 1,
            Message::Decrement => self.value -= 1,
        }
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        Column::new()
            .padding(20)
            .align_items(Align::Center)
            .push(
                Button::new(&mut self.increment_button, Text::new("Increment"))
                    .on_press(Message::Increment),
            )
            .push(Text::new(self.value.to_string()).size(50))
            .push(
                Button::new(&mut self.decrement_button, Text::new("Decrement"))
                    .on_press(Message::Decrement),
            )
            .into()
    }
}

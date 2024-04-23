use iced::widget::{button, column, horizontal_space, row, text, Button, Column, Text};
use iced::alignment::{self, Alignment};
use iced::{Element, Sandbox, Settings};
use rand::prelude::*;
use rand::distributions::Alphanumeric;

struct State {
    chosen_letter: u8,
    was_correct: Option<bool>,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Selected(u8),
}

impl Sandbox for State {
    type Message = Message;

    fn new() -> Self {
        Self {
            chosen_letter: rand::thread_rng().sample(Alphanumeric),
            was_correct: None
        }
    }

    fn title(&self) -> String {
        "Gyro".into()
    }

    fn view(&self) -> Element<Message> {
        let mut new_chars: Vec<u8> = vec![self.chosen_letter];
        let mut rng = rand::thread_rng();

        for _ in 0..3 {
            new_chars.push(rng.sample(Alphanumeric));
        }

        new_chars.shuffle(&mut rng);

        let message = if self.was_correct.is_none() {
            ""
        } else if self.was_correct.unwrap() {
            "Correct"
        } else {
            "NO!!!"
        };

        column![
            row![
                text(message),
            ],
            row![
                button(text(new_chars[0] as char))
                    .on_press(Message::Selected(new_chars[0]))
                    .padding(20), 
                horizontal_space(), 
                button(text(new_chars[1] as char))
                    .on_press(Message::Selected(new_chars[1]))
                    .padding(20), 
            ],
            row![
                text(self.chosen_letter as char)
                    .horizontal_alignment(alignment::Horizontal::Center)
                    .vertical_alignment(alignment::Vertical::Center)
            ],
            row![
                button(text(new_chars[2] as char))
                    .on_press(Message::Selected(new_chars[2]))
                    .padding(20), 
                horizontal_space(), 
                button(text(new_chars[3] as char))
                    .on_press(Message::Selected(new_chars[3]))
                    .padding(20), 
            ],
        ]
        .padding(20)
        .into()
    }

    fn update(&mut self, message: Message) {
        let Message::Selected(letter) = message;
        self.was_correct = Some(letter == self.chosen_letter);
        self.chosen_letter = rand::thread_rng().sample(Alphanumeric);
    }
}

pub fn main() -> iced::Result {
    State::run(Settings::default())
}

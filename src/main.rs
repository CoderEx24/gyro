use iced::alignment::{self, Alignment};
use iced::executor;
use iced::theme::Theme;
use iced::time;
use iced::widget::{button, column, horizontal_space, row, text, Button, Column, Text};
use iced::{Application, Command, Element, Settings, Subscription};

use rand::distributions::Alphanumeric;
use rand::prelude::*;

use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy)]
enum Message {
    Selected(u8),
    Tick(Instant),
}

struct State {
    chosen_letter: u8,
    chars: Vec<u8>,
    was_correct: Option<bool>,
    elapsed_time: Duration,
    timeout: Duration,
    last_tick: Instant,
}

impl Application for State {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_: ()) -> (Self, Command<Message>) {
        let chosen_letter = rand::thread_rng().sample(Alphanumeric);
        let mut chars = vec![chosen_letter];

        for _ in 1..=3 {
            chars.push(rand::thread_rng().sample(Alphanumeric));
        }

        let mut rng = rand::thread_rng();
        chars.shuffle(&mut rng);

        (
            Self {
                chosen_letter,
                chars,
                was_correct: None,
                elapsed_time: Duration::ZERO,
                timeout: Duration::from_secs(10),
                last_tick: Instant::now(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        "Gyro".into()
    }

    fn view(&self) -> Element<Message> {
        let message = if self.was_correct.is_none() {
            ""
        } else if self.was_correct.unwrap() {
            "Correct"
        } else {
            "NO!!!"
        };

        let diff = if self.timeout > self.elapsed_time {
            self.timeout - self.elapsed_time
        } else {
            Duration::ZERO
        };

        if diff != Duration::ZERO {
            column![
                row![
                    text(message),
                    horizontal_space(),
                    text(format!("{:0>2}:{:02}", diff.as_secs(), diff.as_millis())),
                ],
                row![
                    button(text(self.chars[0] as char))
                        .on_press(Message::Selected(self.chars[0]))
                        .padding(20),
                    horizontal_space(),
                    button(text(self.chars[1] as char))
                        .on_press(Message::Selected(self.chars[1]))
                        .padding(20),
                ],
                row![text(self.chosen_letter as char)
                    .horizontal_alignment(alignment::Horizontal::Center)
                    .vertical_alignment(alignment::Vertical::Center)],
                row![
                    button(text(self.chars[2] as char))
                        .on_press(Message::Selected(self.chars[2]))
                        .padding(20),
                    horizontal_space(),
                    button(text(self.chars[3] as char))
                        .on_press(Message::Selected(self.chars[3]))
                        .padding(20),
                ],
            ]
            .padding(20)
            .into()
        } else {
            column![
                text("GAME OVER"),
            ].into()
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        time::every(Duration::from_millis(1)).map(Message::Tick)
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Selected(letter) => {
                let was_correct = letter == self.chosen_letter;
                self.was_correct = Some(was_correct);
                self.chosen_letter = rand::thread_rng().sample(Alphanumeric);

                self.chars[0] = self.chosen_letter;
                let mut rng = rand::thread_rng();

                for i in 1..=3 {
                    self.chars[i] = rng.sample(Alphanumeric);
                }

                self.chars.shuffle(&mut rng);

                if was_correct {
                    self.timeout = std::cmp::max(self.timeout - Duration::from_millis(500), Duration::from_secs(5)); 
                    self.elapsed_time = Duration::ZERO;
                }
            }
            Message::Tick(t) => {
                self.elapsed_time += t - self.last_tick;
                self.last_tick = t;
            }
        }

        Command::none()
    }
}

pub fn main() -> iced::Result {
    State::run(Settings::default())
}

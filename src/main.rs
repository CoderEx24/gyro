use iced::alignment;
use iced::executor;
use iced::theme::Theme;
use iced::time;
use iced::widget::{button, column, horizontal_space, row, text};
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

    correct_clicks: u64,
    wrong_clicks: u64,
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
                correct_clicks: 0,
                wrong_clicks: 0,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        "Gyro".into()
    }

    fn view(&self) -> Element<Message> {
        let diff = if self.timeout > self.elapsed_time {
            self.timeout - self.elapsed_time
        } else {
            Duration::ZERO
        };

        let score = format!("{} Correct | {} Wrong", self.correct_clicks, self.wrong_clicks);

        if diff != Duration::ZERO {
            column![
                row![
                    horizontal_space(),
                    text(score),
                    horizontal_space(),
                    text(format!("{:0>2}:{:02}", diff.as_secs(), diff.as_millis())),
                    horizontal_space(),
                ],
                row![
                    horizontal_space(),
                    button(text(self.chars[0] as char))
                        .on_press(Message::Selected(self.chars[0]))
                        .padding(20),
                    horizontal_space(),
                    button(text(self.chars[1] as char))
                        .on_press(Message::Selected(self.chars[1]))
                        .padding(20),
                    horizontal_space(),
                ],
                row![
                    horizontal_space(),
                    text(self.chosen_letter as char)
                        .horizontal_alignment(alignment::Horizontal::Center)
                        .vertical_alignment(alignment::Vertical::Center),
                    horizontal_space(),
                ],
                row![
                    horizontal_space(),
                    button(text(self.chars[2] as char))
                        .on_press(Message::Selected(self.chars[2]))
                        .padding(20),
                    horizontal_space(),
                    button(text(self.chars[3] as char))
                        .on_press(Message::Selected(self.chars[3]))
                        .padding(20),
                    horizontal_space(),
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
                    self.correct_clicks += 1;
                } else {
                    self.wrong_clicks += 1;
                }

                self.timeout = std::cmp::max(
                    self.timeout - Duration::from_millis(500),
                    Duration::from_secs(5),
                );
                self.elapsed_time = Duration::ZERO;
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
    let mut settings = Settings::default();

    settings.window.size.width = 400f32;
    settings.window.size.height = 400f32;
    settings.window.resizable = false;

    State::run(settings)
}

use iced::alignment;
use iced::executor;
use iced::theme::Theme;
use iced::time;
use iced::widget::{button, column, horizontal_space, row, text, vertical_space};
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

    correct_clicks: u32,
    wrong_clicks: u32,
    stats: Vec<(u32, u32)>,
    level: u8,
}

impl Application for State {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_: ()) -> (Self, Command<Message>) {
        let chosen_letter = rand::thread_rng().sample(Alphanumeric);
        let mut chars = vec![chosen_letter];

        let mut rng = rand::thread_rng();

        chars.extend(Alphanumeric.sample_iter(&mut rng).take(3).map(u8::from));

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
                stats: vec![(0, 0); 3],
                level: 1,
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

        let score = format!(
            "{} Correct | {} Wrong",
            self.correct_clicks, self.wrong_clicks
        );

        if self.level < 4 {
            column![
                row![
                    horizontal_space(),
                    text(score),
                    horizontal_space(),
                    text(format!("{:0>2}:{:02}", diff.as_secs(), diff.as_millis())),
                    horizontal_space(),
                    vertical_space(),
                ],
                row![
                    horizontal_space(),
                    button(text(self.chars[0] as char))
                        .on_press(Message::Selected(self.chars[0]))
                        .padding(20),
                    horizontal_space(),
                    vertical_space(),
                ],
                row![
                    button(text(self.chars[1] as char))
                        .on_press(Message::Selected(self.chars[1]))
                        .padding(20),
                    horizontal_space(),
                    text(self.chosen_letter as char)
                        .horizontal_alignment(alignment::Horizontal::Center)
                        .vertical_alignment(alignment::Vertical::Center),
                    vertical_space(),
                    horizontal_space(),
                    button(text(self.chars[2] as char))
                        .on_press(Message::Selected(self.chars[2]))
                        .padding(20),
                ],
                row![
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
                text("GAME OVER\nStats"),
                row![]
                    .extend(self.stats.iter().enumerate().map(|(i, (c, w))| {
                        row![
                            column![
                                text(format!("Level {}", i)),
                                text(format!("Correct {}\nWrong {}", c, w))
                            ],
                            horizontal_space()
                        ]
                        .padding(10)
                        .into()
                    }))
                    .padding(7)
            ]
            .into()
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

                let mut rng = rand::thread_rng();

                self.chars.clear();
                self.chars.push(self.chosen_letter);
                self.chars
                    .extend(Alphanumeric.sample_iter(&mut rng).take(3).map(u8::from));

                self.chars.shuffle(&mut rng);

                if was_correct {
                    self.correct_clicks += 1;
                } else {
                    self.wrong_clicks += 1;
                }
            }
            Message::Tick(t) => {
                self.elapsed_time += t - self.last_tick;
                self.last_tick = t;

                if self.elapsed_time > self.timeout && self.level < 4 {
                    self.elapsed_time = Duration::ZERO;
                    self.timeout = match self.level {
                        1 => Duration::from_secs(10),
                        2 => Duration::from_secs(7),
                        3 => Duration::from_secs(5),
                        _ => Duration::ZERO,
                    };

                    self.stats[(self.level - 1) as usize].0 = self.correct_clicks;
                    self.stats[(self.level - 1) as usize].1 = self.wrong_clicks;

                    self.correct_clicks = 0;
                    self.wrong_clicks = 0;
                    self.level += 1;
                }
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

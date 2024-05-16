use iced::alignment;
use iced::executor;
use iced::theme::Theme;
use iced::time;
use iced::widget::{
    button, column, container, horizontal_space, mouse_area, progress_bar, row, text,
    vertical_space,
};
use iced::{Application, Command, Element, Settings, Subscription};

use rand::distributions::Alphanumeric;
use rand::prelude::*;

use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy)]
enum Message {
    MouseEntered(char),
    MouseLeft,
    Tick(Instant),
    Reset,
}

struct State {
    chosen_letter: char,
    chars: Vec<char>,
    was_correct: Option<bool>,
    currently_on: Option<char>,
    elapsed_time: Duration,
    timeout: Duration,
    mouse_on_durations: [Duration; 4],
    last_tick: Instant,

    correct_clicks: u32,
    wrong_clicks: u32,
    missed_clicks: u32,
    stats: Vec<Option<(u32, u32, u32)>>,
    level: u8,
}

impl Application for State {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_: Self::Flags) -> (Self, Command<Message>) {
        let chosen_letter = rand::thread_rng().sample(Alphanumeric) as char;
        let mut chars = vec![chosen_letter];

        let mut rng = rand::thread_rng();

        chars.extend(
            Alphanumeric
                .sample_iter(&mut rng)
                .filter(|&c| c as char != chosen_letter)
                .take(3)
                .map(char::from),
        );

        chars.shuffle(&mut rng);

        (
            Self {
                chosen_letter,
                chars,
                was_correct: None,
                elapsed_time: Duration::ZERO,
                currently_on: None,
                timeout: Duration::from_secs(10),
                mouse_on_durations: [Duration::ZERO; 4],
                last_tick: Instant::now(),
                correct_clicks: 0,
                wrong_clicks: 0,
                missed_clicks: 0,
                stats: vec![None; 3],
                level: 0,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        "Gyro".into()
    }

    fn theme(&self) -> Self::Theme {
        Self::Theme::Dracula
    }

    fn view(&self) -> Element<Message> {
        let diff = if self.timeout > self.elapsed_time {
            self.timeout - self.elapsed_time
        } else {
            Duration::ZERO
        };

        let score = format!(
            "{} Correct | {} Wrong | {} Missed",
            self.correct_clicks, self.wrong_clicks, self.missed_clicks
        );

        let (level, trial) = (self.level / 10 + 1, self.level % 10 + 1);

        let level = format!("Level {}, Trial {}", level, trial);

        let ratios: Vec<f32> = self
            .mouse_on_durations
            .iter()
            .map(|d| d.as_secs_f32() * 1000f32)
            .collect();

        // {{{ view logic
        if self.level < 30 {
            column![
                row![
                    text(score),
                    horizontal_space(),
                    text(level),
                    vertical_space(),
                ],
                row![
                    horizontal_space(),
                    text(format!("Time: {:0>2}", diff.as_secs())),
                    horizontal_space(),
                    progress_bar(
                        0f32..=self.timeout.as_secs_f32(),
                        self.elapsed_time.as_secs_f32()
                    ),
                    vertical_space(),
                ]
                .align_items(alignment::Horizontal::Center.into()),
                row![
                    horizontal_space(),
                    column![
                        mouse_area(container(text(self.chars[0])).padding(20))
                            .on_enter(Message::MouseEntered(self.chars[0]))
                            .on_exit(Message::MouseLeft),
                        progress_bar(0f32..=700f32, ratios[0]).height(8),
                    ]
                    .align_items(alignment::Vertical::Center.into()),
                    horizontal_space(),
                    vertical_space(),
                ],
                row![
                    column![
                        mouse_area(container(text(self.chars[1])).padding(20))
                            .on_enter(Message::MouseEntered(self.chars[1]))
                            .on_exit(Message::MouseLeft),
                        progress_bar(0f32..=700f32, ratios[1]).height(8),
                    ]
                    .align_items(alignment::Vertical::Center.into()),
                    horizontal_space(),
                    container(text(self.chosen_letter))
                        .padding(30)
                        .align_x(alignment::Horizontal::Center)
                        .align_y(alignment::Vertical::Center)
                        .center_x()
                        .center_y(),
                    vertical_space(),
                    horizontal_space(),
                    column![
                        mouse_area(container(text(self.chars[2])).padding(20))
                            .on_enter(Message::MouseEntered(self.chars[2]))
                            .on_exit(Message::MouseLeft),
                        progress_bar(0f32..=700f32, ratios[2]).height(8),
                    ]
                    .align_items(alignment::Vertical::Center.into()),
                ],
                row![
                    horizontal_space(),
                    column![
                        mouse_area(container(text(self.chars[3])).padding(20))
                            .on_enter(Message::MouseEntered(self.chars[3]))
                            .on_exit(Message::MouseLeft),
                        progress_bar(0f32..=700f32, ratios[3]).height(8),
                    ]
                    .align_items(alignment::Vertical::Center.into()),
                    horizontal_space(),
                ],
            ]
            .padding(20)
            .into()
        } else {
            column![
                row![horizontal_space(), text("GAME OVER"), horizontal_space(),],
                row![horizontal_space(), text("Stats"), horizontal_space(),],
                row![]
                    .extend(
                        self.stats
                            .iter()
                            .map(|o| o.unwrap_or((999, 999, 999)))
                            .enumerate()
                            .map(|(i, (c, w, m))| {
                                row![
                                    column![
                                        text(format!("Level {}", i + 1)),
                                        text(format!("Correct {}", c)),
                                        text(format!("Wrong {}", w)),
                                        text(format!("Missed {}", m)),
                                    ],
                                    horizontal_space()
                                ]
                                .padding(10)
                                .into()
                            })
                    )
                    .padding(7),
                row![
                    horizontal_space(),
                    button("Reset").on_press(Message::Reset),
                    horizontal_space(),
                ]
            ]
            .spacing(50)
            .padding(20)
            .into()
            // }}}
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        time::every(Duration::from_millis(1)).map(Message::Tick)
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Reset => {
                self.level = 0;
                self.elapsed_time = Duration::ZERO;
                self.last_tick = Instant::now();
                self.timeout = Duration::from_secs(10);
                self.currently_on = None;
                self.mouse_on_durations.fill(Duration::ZERO);
                self.stats.fill(None);
                self.correct_clicks = 0;
                self.wrong_clicks = 0;
                self.missed_clicks = 0;
            }
            Message::MouseEntered(letter) => {
                self.currently_on = Some(letter);
            }
            Message::MouseLeft => {
                self.currently_on = None;
                self.mouse_on_durations.fill(Duration::ZERO);
            }
            Message::Tick(t) => {
                let diff = t - self.last_tick;
                self.elapsed_time += diff;
                self.last_tick = t;

                if self.currently_on.is_some() {
                    let letter = self.currently_on.unwrap();
                    let idx = self.chars.iter().position(|&c| c == letter).unwrap();
                    self.mouse_on_durations[idx] += diff;

                    if self.mouse_on_durations[idx] > Duration::from_millis(700) {
                        let was_correct = letter == self.chosen_letter;
                        self.was_correct = Some(was_correct);
                        self.chosen_letter = rand::thread_rng().sample(Alphanumeric) as char;

                        let mut rng = rand::thread_rng();

                        self.chars.clear();
                        self.chars.push(self.chosen_letter);
                        self.chars.extend(
                            Alphanumeric
                                .sample_iter(&mut rng)
                                .filter(|&c| c as char != self.chosen_letter)
                                .take(3)
                                .map(char::from),
                        );

                        self.chars.shuffle(&mut rng);

                        if was_correct {
                            self.correct_clicks += 1;
                        } else {
                            self.wrong_clicks += 1;
                        }

                        self.currently_on = Some(self.chars[idx]);
                        self.mouse_on_durations.fill(Duration::ZERO);
                        self.elapsed_time = Duration::ZERO;
                        self.level += 1;
                    }
                }

                if self.elapsed_time > self.timeout && self.level < 30 {
                    self.missed_clicks += 1;
                    self.elapsed_time = Duration::ZERO;
                    self.level += 1;
                }

                if self.level % 10 == 0 && self.level > 0 {
                    let idx = (self.level / 10 - 1) as usize;

                    if idx < self.stats.len() && self.stats[idx].is_none() {
                        self.timeout = match self.level {
                            00..=09 => Duration::from_secs(10),
                            10..=19 => Duration::from_secs(7),
                            20..=29 => Duration::from_secs(5),
                            _ => Duration::ZERO,
                        };

                        self.stats[idx] =
                            Some((self.correct_clicks, self.wrong_clicks, self.missed_clicks));

                        self.correct_clicks = 0;
                        self.wrong_clicks = 0;
                        self.missed_clicks = 0;
                    }
                }
            }
        }

        Command::none()
    }
}

pub fn main() -> iced::Result {
    let mut settings = Settings::default();

    settings.window.size.width = 600f32;
    settings.window.size.height = 600f32;
    settings.window.resizable = false;
    settings.window.exit_on_close_request = true;

    State::run(settings)
}

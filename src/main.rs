use iced::{
    alignment,
    widget::{Button, Column, Row, Switch, Text, TextInput},
    Alignment, Element, Sandbox, Settings,
};

fn main() -> iced::Result {
    RandNumberGen::run(Settings::default())
}

struct RandNumberGen {
    min: String,
    max: String,
    include_max: bool,
    random_number: Option<i32>,
}

#[derive(Debug, Clone)]
enum Message {
    MinChanged(String),
    MaxChanged(String),
    IncludeMaxToggled(bool),
    GeneratePressed,
}

impl Sandbox for RandNumberGen {
    type Message = Message;

    fn new() -> Self {
        Self {
            min: String::new(),
            max: String::new(),
            include_max: true,
            random_number: None,
        }
    }

    fn title(&self) -> String {
        String::from("Random Number Generator")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::MinChanged(s) => self.min = s,
            Message::MaxChanged(s) => self.max = s,
            Message::IncludeMaxToggled(b) => self.include_max = b,
            Message::GeneratePressed => {
                let min_val = self.min.parse().unwrap_or(0);
                let max_val = self.max.parse().unwrap_or(0);

                let (lower, upper) = if min_val <= max_val {
                    (min_val, max_val)
                } else {
                    (max_val, min_val)
                };

                self.random_number = if self.include_max {
                    Some(fastrand::i32(lower..=upper))
                } else {
                    if lower < upper {
                        Some(fastrand::i32(lower..upper))
                    } else {
                        None
                    }
                };
            }
        }
    }

    fn view(&self) -> Element<Message> {
        Column::new()
            .padding(20)
            .spacing(20)
            .align_items(Alignment::Center)
            .push(
                Column::new()
                    .spacing(10)
                    .push(Text::new("Minimum Value").size(16))
                    .push(
                        TextInput::new("Enter minimum...", &self.min)
                            .on_input(Message::MinChanged)
                            .padding(10),
                    ),
            )
            .push(
                Column::new()
                    .spacing(10)
                    .push(Text::new("Maximum Value").size(16))
                    .push(
                        TextInput::new("Enter maximum...", &self.max)
                            .on_input(Message::MaxChanged)
                            .padding(10),
                    ),
            )
            .push(
                Row::new()
                    .spacing(10)
                    .align_items(Alignment::Center)
                    .push(Switch::new(self.include_max, Message::IncludeMaxToggled))
                    .push(Text::new("Include upper bound")),
            )
            .push(
                Button::new(Text::new("Generate Random Number").size(18))
                    .on_press(Message::GeneratePressed)
                    .padding(10),
            )
            .push(
                match self.random_number {
                    Some(n) => Text::new(n.to_string()).size(48),
                    None => Text::new("Click Generate")
                        .size(32)
                        .style(iced::theme::Text::Color(iced::Color::from_rgb8(100, 100, 100))),
                }
                .horizontal_alignment(alignment::Horizontal::Center),
            )
            .into()
    }
}
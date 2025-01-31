use iced::{
    font::{Family, Stretch, Style, Weight},
    widget::{button, column, row, text, text_input},
    Alignment, Element, Font, Theme,
};

fn main() -> iced::Result {
    let theme = |_s: &RandyGen| Theme::CatppuccinMocha;

    iced::application("Randy", RandyGen::update, RandyGen::view)
        .theme(theme)
        .centered()
        .run()
}

#[derive(Default)]
struct RandyGen {
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

impl RandyGen {
    fn update(&mut self, message: Message) {
        match message {
            Message::MinChanged(s) => self.min = s,
            Message::MaxChanged(s) => self.max = s,
            Message::IncludeMaxToggled(b) => self.include_max = b,
            Message::GeneratePressed => {
                let min_val = self.min.parse().unwrap_or(0);
                let max_val = self.max.parse().unwrap_or(0);

                // Swap min and max if min > max
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
        // Create the "Minimum Value" section
        let min_value_section = column![
            text("Minimum Value").size(16),
            text_input("Enter minimum...", &self.min)
                .on_input(Message::MinChanged)
                .padding(10),
        ]
        .spacing(10);

        // Create the "Maximum Value" section
        let max_value_section = column![
            text("Maximum Value").size(16),
            text_input("Enter maximum...", &self.max)
                .on_input(Message::MaxChanged)
                .padding(10),
        ]
        .spacing(10);

        // Create the "Include upper bound" button row
        let include_upper_bound_button = row![
            button(text(if self.include_max {
                "Including upper bound"
            } else {
                "Excluding upper bound"
            }))
            .on_press(Message::IncludeMaxToggled(!self.include_max)) // Toggle the state
        ]
        .spacing(10)
        .align_y(Alignment::Center);

        // Create the "Generate Random Number" button
        let generate_random_button = button(text("Generate Random Number"))
            .on_press(Message::GeneratePressed)
            .padding(10);

        let number_style = Font {
            family: Family::SansSerif,
            weight: Weight::Bold,
            stretch: Stretch::Normal,
            style: Style::Normal,
        };

        // Create the random number or placeholder text display
        let random_number_display = match self.random_number {
            Some(n) => text(n.to_string()).size(72).font(number_style),
            None => text("Click Generate").size(32),
        };

        // Combine all sections into a final Column
        column![
            random_number_display,
            generate_random_button,
            max_value_section,
            min_value_section,
            include_upper_bound_button,
        ]
        .padding(20)
        .spacing(20)
        .align_x(Alignment::Center)
        .into()
    }
}

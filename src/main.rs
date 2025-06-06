use iced::{Theme, Length, widget::{Button, Column, Container, Text}};

mod capture;
mod editing;

#[derive(Debug, Clone, Copy)]
enum Message {
    ToggleRecording,
    Screenshot,
}

pub fn main() -> iced::Result {
    iced::application("SnipStream", update, view)
        .theme(|_| Theme::Dark)
        .run()
}

fn update(recording: &mut bool, message: Message) {
    match message {
        Message::ToggleRecording => {
            if *recording {
                capture::stop_capture();
            } else {
                capture::start_capture();
            }
            *recording = !*recording;
        }
        Message::Screenshot => {
            capture::save_screenshot_auto();
        }
    }
}

fn view(recording: &bool) -> Column<Message> {
    let record_label = if *recording { "Stop" } else { "Record" };

    let controls = Column::new()
        .spacing(20)
        .push(Button::new(Text::new(record_label)).on_press(Message::ToggleRecording))
        .push(Button::new(Text::new("Screenshot")).on_press(Message::Screenshot));

    let layout = Column::new()
        .padding(40)
        .spacing(20)
        .push(
            Container::new(Text::new("SnipStream").size(40))
                .center_x(Length::Shrink),
        )
        .push(controls);

    Column::new()
        .push(
            Container::new(layout)
                .center_x(Length::Fill)
                .center_y(Length::Fill),
        )
}



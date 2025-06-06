use iced::{Theme, Length, widget::{Button, Column, Container, Text}};

#[cfg(not(target_os = "windows"))]
compile_error!("SnipStream currently supports only Windows");

mod capture;
mod editing;

#[derive(Debug, Clone, Copy)]
enum Message {
    ToggleRecording,
    TogglePause,
    Screenshot,
}

pub fn main() -> iced::Result {
    iced::application((false, false), update, view)
        .theme(|_| Theme::Dark)
        .run()
}

fn update(state: &mut (bool, bool), message: Message) {
    match message {
        Message::ToggleRecording => {
            if state.0 {
                capture::stop_capture();
            } else {
                capture::start_capture();
            }
            state.0 = !state.0;
            if !state.0 {
                state.1 = false;
            }
        }
        Message::TogglePause => {
            if state.0 {
                if state.1 {
                    capture::resume_capture();
                } else {
                    capture::pause_capture();
                }
                state.1 = !state.1;
            }
        }
        Message::Screenshot => {
            capture::save_screenshot_auto();
        }
    }
}

fn view(state: &(bool, bool)) -> Column<Message> {
    let record_label = if state.0 { "Stop" } else { "Record" };
    let pause_label = if state.1 { "Resume" } else { "Pause" };

    let controls = Column::new()
        .spacing(20)
        .push(Button::new(Text::new(record_label)).on_press(Message::ToggleRecording));
    let controls = if state.0 {
        controls
            .push(Button::new(Text::new(pause_label)).on_press(Message::TogglePause))
            .push(Button::new(Text::new("Screenshot")).on_press(Message::Screenshot))
    } else {
        controls.push(Button::new(Text::new("Screenshot")).on_press(Message::Screenshot))
    };

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



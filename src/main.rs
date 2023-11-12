mod clipboard_helper;
mod component;
mod data_struct;
mod event_handler;
mod file_handler;
mod global;
mod layout;
mod view;
use data_struct::ClipPop;
mod date_helper;
use iced::executor;
use iced::{Application, Command, Element, Settings, Theme};

pub fn main() -> iced::Result {
    ClipPop::run(Settings {
        antialiasing: false,
        window: iced::window::Settings {
            size: (global::MAX_SCREEN_WIDTH, global::MAX_SCREEN_HEIGHT),
            resizable: true,
            level: iced::window::Level::AlwaysOnTop,
            decorations: true,
            visible: true,
            position: iced::window::Position::Specific(0, 0),
            ..iced::window::Settings::default()
        },
        ..Settings::default()
    })
}

impl Application for ClipPop {
    type Message = event_handler::Message;
    type Flags = ();
    type Executor = executor::Default;
    type Theme = Theme;

    fn new(_flags: Self::Flags) -> (Self, Command<event_handler::Message>) {
        (
            Self {
                clipboard_contents: Vec::new(),
                show_date_picker: false,
                current_date: chrono::Local::now(),
            },
            Command::perform(initialize_app(), |_| {
                event_handler::Message::IntializeTheScreen
            }),
        )
    }

    fn theme(&self) -> Self::Theme {
        Theme::Dark
    }

    fn title(&self) -> String {
        String::from("ClipPop")
    }

    fn update(&mut self, message: event_handler::Message) -> Command<event_handler::Message> {
        event_handler::handle_message(self, message)
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        iced::time::every(std::time::Duration::from_millis(500)).map(|_| {
            event_handler::Message::ClipboardUpdated(clipboard_helper::get_clipboard_text())
        })
    }

    fn style(&self) -> <Self::Theme as iced::application::StyleSheet>::Style {
        <Self::Theme as iced::application::StyleSheet>::Style::default()
    }

    fn view(&self) -> Element<event_handler::Message> {
        view::app(self.to_owned())
    }
}

async fn initialize_app() -> Result<(), data_struct::Error> {
    Ok(())
}

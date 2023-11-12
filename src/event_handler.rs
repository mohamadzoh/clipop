use crate::{clipboard_helper, data_struct, date_helper, file_handler, global};
use chrono::{DateTime, Local};
use iced::{Command, Size};
use iced_aw::date_picker;

#[derive(Debug, Clone)]
pub enum Message {
    ClipboardUpdated(Option<String>),
    IntializeTheScreen,
    CopyToClipboard(String),
    ClearClipboard,
    ScreenSize(u32, u32),
    ChooseDate,
    LoadHistory(date_picker::Date),
    CancelDate,
}

pub fn content_to_display_content(content: String) -> String {
    let mut max_character: usize = content.len();
    if max_character > 100 {
        max_character = 100;
    }
    content[..max_character].trim().to_string()
}

pub fn handle_message(
    application: &mut data_struct::ClipPop,
    message: Message,
) -> Command<Message> {
    match message {
        Message::ClipboardUpdated(content) => {
            match content {
                Some(clip) => {
                    if application.clipboard_contents.len() > 0 {
                        if application.clipboard_contents.last().unwrap().content == clip {
                            return Command::none();
                        }
                    }
                    application
                        .clipboard_contents
                        .push(data_struct::ClipboardContent {
                            content: clip.clone(),
                            display_text: content_to_display_content(clip.clone()),
                            content_type: "Text".to_string(),
                        });
                    file_handler::write_to_file(clip, None);
                }
                None => {}
            };
            Command::none()
        }
        Message::IntializeTheScreen => {
            file_handler::read_from_file(None).iter().for_each(|clip| {
                application
                    .clipboard_contents
                    .push(data_struct::ClipboardContent {
                        content: clip.clone(),
                        display_text: content_to_display_content(clip.clone()),
                        content_type: "Text".to_string(),
                    })
            });
            Command::batch(vec![
                iced::window::maximize::<Message>(true),
                iced::window::fetch_size(|size| Message::ScreenSize(size.width, size.height)),
                iced::window::maximize::<Message>(false),
            ])
        }
        Message::CopyToClipboard(text) => {
            clipboard_helper::write_clipboard_text(text);
            Command::none()
        }
        Message::ClearClipboard => {
            clipboard_helper::clear_clipboard();
            application.clipboard_contents = [].to_vec();

            Command::none()
        }
        Message::ScreenSize(width, _height) => Command::batch(vec![
            iced::window::resize::<Message>(Size::new(
                global::MAX_SCREEN_WIDTH,
                global::MAX_SCREEN_HEIGHT,
            )),
            iced::window::move_to::<Message>(
                global::screen_left_position(width),
                global::SCREEN_TOP_PADDING,
            ),
        ]),
        Message::LoadHistory(date) => {
            application.clipboard_contents.clear();
            application.show_date_picker = false;
            let date_time: DateTime<Local> =
                date_helper::from_datetime_picker_to_chrono_date(date.year, date.month, date.day);
            application.current_date = date_time;
            file_handler::read_from_file(Some(date_time))
                .iter()
                .for_each(|clip| {
                    application
                        .clipboard_contents
                        .push(data_struct::ClipboardContent {
                            content: clip.clone(),
                            display_text: content_to_display_content(clip.clone()),
                            content_type: "Text".to_string(),
                        })
                });
            Command::none()
        }
        Message::ChooseDate => {
            application.show_date_picker = true;
            Command::none()
        }
        Message::CancelDate => {
            application.show_date_picker = false;
            Command::none()
        }
    }
}

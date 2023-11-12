use crate::{
    component::{svg_icon, text_button},
    data_struct, event_handler, global, layout,
};
use iced::{
    widget::{Column, Row, Scrollable, TextInput},
    Alignment, Element, Length, Renderer,
};

pub fn clipboard_entry_views<'a>(
    clipboard_contents: Vec<data_struct::ClipboardContent>,
) -> Vec<Element<'a, event_handler::Message, Renderer>> {
    clipboard_contents
        .iter()
        .rev()
        .map(|clip| clipboard_item(clip))
        .collect()
}

pub fn clipboard_item<'a>(
    clipboard_content: &data_struct::ClipboardContent,
) -> Element<'a, event_handler::Message, Renderer> {
    let text_input = TextInput::new("", &clipboard_content.display_text)
        .width(Length::Fill)
        .line_height(2.0)
        .size(10);

    Row::new()
        .align_items(Alignment::Center)
        .push(text_input)
        .spacing(10)
        .push(text_button(
            svg_icon(global::COPY_ICON),
            event_handler::Message::CopyToClipboard(clipboard_content.content.clone()),
        ))
        .padding(0)
        .into()
}

pub fn app(
    application: data_struct::ClipPop,
) -> Element<'static, event_handler::Message, Renderer> {
    let clipboard_entries = clipboard_entry_views(application.clipboard_contents.clone());
    let clipboard_contents = Column::with_children(clipboard_entries)
        .spacing(5)
        .padding(10);

    let scrollable_clipboard_contents = Scrollable::new(clipboard_contents)
        .width(Length::Fill)
        .height(Length::Fill);

    layout::view_layout(application, scrollable_clipboard_contents.into())
}

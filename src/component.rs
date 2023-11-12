use iced::{
    theme,
    widget::{button, svg},
    Element,
};

use crate::event_handler;

pub fn svg_icon(icon: &[u8]) -> Element<'static, event_handler::Message> {
    let handle = svg::Handle::from_memory(icon.to_vec());
    svg::Svg::new(handle).width(24).height(24).into()
}

pub fn text_button(
    content: Element<'static, event_handler::Message>,
    message: event_handler::Message,
) -> Element<'static, event_handler::Message> {
    button::Button::new(content)
        .on_press(message)
        .padding(0)
        .style(theme::Button::Text)
        .into()
}

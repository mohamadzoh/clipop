use crate::{
    component::{svg_icon, text_button},
    data_struct, event_handler, global,
};
use iced::{
    widget::{tooltip, Column, Row, Text},
    Alignment, Element,
};
use iced_aw::date_picker;

pub fn view_layout(
    application: data_struct::ClipPop,
    body: Element<event_handler::Message>,
) -> Element<event_handler::Message> {
    Column::new()
        .align_items(Alignment::Center)
        .push(control_panel(&application))
        .push(Text::new(application.current_date.to_rfc2822()).size(10))
        .spacing(0)
        .push(body)
        .into()
}

pub fn control_panel(
    application: &data_struct::ClipPop,
) -> Element<'static, event_handler::Message> {
    Row::new()
        .push(clear_button())
        .push(history_button(&application))
        .padding(10)
        .into()
}

pub fn control_panel_button(
    content: Element<'static, event_handler::Message>,
    message: event_handler::Message,
    tooltip_message: &str,
) -> Element<'static, event_handler::Message> {
    tooltip(
        text_button(content, message),
        tooltip_message,
        tooltip::Position::Right,
    )
    .into()
}

pub fn clear_button() -> Element<'static, event_handler::Message> {
    control_panel_button(
        svg_icon(global::CLEAR_ICON),
        event_handler::Message::ClearClipboard,
        "Clear clipboard",
    )
}

pub fn history_button(
    application: &data_struct::ClipPop,
) -> Element<'static, event_handler::Message> {
    let datepicker = date_picker(
        application.show_date_picker,
        application.current_date.date_naive(),
        control_panel_button(
            svg_icon(global::HISTORY_ICON),
            event_handler::Message::ChooseDate,
            "Choose date",
        ),
        event_handler::Message::CancelDate,
        event_handler::Message::LoadHistory,
    );
    datepicker.into()
}

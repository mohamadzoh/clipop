#[derive(Debug, Clone)]
pub struct ClipboardContent {
    pub content: String,
    pub display_text: String,
    pub content_type: String,
}

#[derive(Default, Debug, Clone)]
pub struct ClipPop {
    pub clipboard_contents: Vec<ClipboardContent>,
    pub show_date_picker: bool,
    pub current_date: chrono::DateTime<chrono::Local>,
}

#[derive(Debug, Clone)]
pub enum Error {}

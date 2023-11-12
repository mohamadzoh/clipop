use arboard::Clipboard;
pub fn get_clipboard_text() -> Option<String> {
    let mut clipboard = Clipboard::new().unwrap();
    if let Ok(content) = clipboard.get_text() {
        if !content.is_empty() && !unaccepted_characters().contains(&content) {
            Some(content)
        } else {
            None
        }
    } else {
        None
    }
}

pub fn write_clipboard_text(text: String) {
    let mut clipboard = Clipboard::new().unwrap();
    clipboard.set_text(text).unwrap();
}

pub fn clear_clipboard() {
    let mut clipboard = Clipboard::new().unwrap();
    let _ = clipboard.clear();
}

fn unaccepted_characters() -> Vec<String> {
    ['\n', '\r', '\t', '\0', '\x0B', '\x0C', '\u{FEFF}', ' ']
        .map(|c| c.to_string())
        .to_vec()
}

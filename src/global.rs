pub const MAX_SCREEN_WIDTH: u32 = 300;
pub const MAX_SCREEN_HEIGHT: u32 = 500;

pub const SCREEN_TOP_PADDING: i32 = 40;

pub const CLEAR_ICON: &[u8] = include_bytes!("./assets/icons/clear.svg");
pub const COPY_ICON: &[u8] = include_bytes!("./assets/icons/copy.svg");
pub const HISTORY_ICON: &[u8] = include_bytes!("./assets/icons/history.svg");

pub fn screen_left_position(width: u32) -> i32 {
    (width as f32 - (MAX_SCREEN_WIDTH as f32 * 2.31)) as i32
}

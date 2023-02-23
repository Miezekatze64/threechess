#[derive(Clone, Copy)]
pub struct Color(pub u8, pub u8, pub u8);

impl Color {
    pub const WHITE: Self = Color(0xff, 0xff, 0xff);
    pub const RED: Self = Color(0xff, 0x00, 0x00);
    pub const GREEN: Self = Color(0x00, 0xff, 0x00);
    pub const YELLOW: Self = Color(0xff, 0xff, 0x00);
    pub const BLACK: Self = Color(0x00, 0x00, 0x00);
}

#[derive(PartialEq, Eq, Debug)]
pub enum MouseButton {
    Left,
    Unknown,
}

#[derive(Debug)]
pub enum Event {
    Quit,
    MouseButtonUp(MouseButton, i32, i32)
}

pub trait Backend {
    fn draw_line(&mut self, a: (i32, i32), b: (i32, i32));
    fn set_draw_color(&mut self, color: Color);
    fn clear(&mut self);
    fn win_size(&self) -> (u32, u32);
    fn poll_event(&mut self) -> Vec<Event>;
    fn draw_point(&mut self, x: i32, y: i32);
    fn text_size(&self, string: &str) -> (u32, u32);
    fn render_text(&mut self, text: &str, x: i32, y: i32, color: Color);
    fn render_png_data(&mut self, data: &[u8], x: i32, y: i32, w: i32, h: i32);
    fn present(&mut self);

    fn read_file(&self, path: &str) -> Vec<u8>;

    fn log(string: String);
    fn elog(string: String);
}

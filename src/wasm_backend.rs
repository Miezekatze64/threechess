use crate::backend::{Backend,MouseButton,Event};

pub struct WasmBackend;

impl WasmBackend {
    pub fn new() -> Self {
        Self
    }
}

extern "C" {
    fn _log(ptr: *const u8, len: usize);
    fn _elog(ptr: *const u8, len: usize);
    fn _read_file(ptr: *const u8, len: usize, buf: *mut u8, buf_len: usize);
    fn _get_file_size(ptr: *const u8, len: usize) -> usize;
    fn _set_draw_color(r: u8, g: u8, b: u8);
    fn _clear();
    fn _win_size(w: *mut u32, h: *mut u32);
    fn _event_queue_size() -> usize;
    fn _get_event_queue(data: *mut u8);
    fn _text_size(txt_ptr: *const u8, txt_len: usize, w: *mut u32, h: *mut u32);
    fn _render_text(txt_ptr: *const u8, txt_len: usize, x: i32, y: i32,
                    r: u8, g: u8, b: u8);
    fn _draw_point(x: i32, y: i32);
    fn _draw_line(x1: i32, y1: i32, x2: i32, y2: i32);
    fn _render_png(data_ptr: *const u8, data_len: usize, x: i32, y: i32, w: i32, h: i32);
    fn _present();
}

macro_rules! read_i32 {
    ($b:expr, $idx:ident) => {{
        let __bytes: [u8; 4] = $b
            .iter().cloned().skip($idx)
                            .take(4)
                            .collect::<Vec<_>>()
            .try_into().unwrap();
        $idx += 4;

        let __val = i32::from_le_bytes(__bytes);
        __val
    }};
}


impl Backend for WasmBackend {
    fn draw_line(&mut self, a: (i32, i32), b: (i32, i32)) {
        unsafe {_draw_line(a.0, a.1, b.0, b.1)};
    }

    fn set_draw_color(&mut self, color: crate::backend::Color) {
        unsafe {_set_draw_color(color.0, color.1, color.2)};
    }

    fn clear(&mut self) {
        unsafe {_clear()};
    }

    fn win_size(&self) -> (u32, u32) {
        let mut w: u32 = 0;
        let mut h: u32 = 0;
        unsafe {_win_size(&mut w as *mut _, &mut h as *mut _)};
        (w ,h)
    }

    fn poll_event(&mut self) -> Vec<crate::backend::Event> {
        let len = unsafe {_event_queue_size()};
        let mut data = vec![];
        data.resize(len, 0);

        unsafe {_get_event_queue(data.as_mut_ptr())};
        let mut events = vec![];

        const EVENT_TYPE_MOUSE_UP: i32 = 0;
        let mut idx = 0;
        while idx < data.len() {
            let tp = read_i32!(data, idx);
            match tp {
                EVENT_TYPE_MOUSE_UP => {
                    let btn = read_i32!(data, idx);
                    assert_eq!(btn, 0);
                    let x = read_i32!(data, idx);
                    let y = read_i32!(data, idx);
                    events.push(Event::MouseButtonUp(MouseButton::Left, x, y));
                },
                _ => unreachable!("invalid event type: {tp}"),
            }
        }
        events
    }

    fn draw_point(&mut self, x: i32, y: i32) {
        unsafe {_draw_point(x, y)};
    }

    fn text_size(&self, string: &str) -> (u32, u32) {
        let mut w: u32 = 0;
        let mut h: u32 = 0;
        unsafe {_text_size(string.as_ptr(),
                           string.bytes().len(),
                           &mut w as *mut _, &mut h as *mut _)};
        (w ,h)
    }

    fn render_text(&mut self, text: &str, x: i32, y: i32, color: crate::backend::Color) {
        unsafe {_render_text(text.as_ptr(), text.bytes().len(), x, y,
                    color.0, color.1, color.2)};
    }

    fn render_png_data(&mut self, data: &[u8], x: i32, y: i32, w: i32, h: i32) {
        unsafe {_render_png(data.as_ptr(), data.len(), x, y, w, h)};
    }

    fn present(&mut self) {
        unsafe {_present()};
    }

    fn log(string: String) {
        unsafe {_log(string.as_ptr(), string.bytes().count())};
    }

    fn elog(string: String) {
        unsafe {_elog(string.as_ptr(), string.bytes().count())};
    }

    fn read_file(&self, path: &str) -> Vec<u8> {
        let size = unsafe {_get_file_size(path.as_ptr(), path.bytes().count())};

        let mut buf = vec![];
        buf.resize(size, 0);
        unsafe {_read_file(path.as_ptr(), path.bytes().count(),
                           buf.as_mut_ptr(), size)};
        buf
    }
}

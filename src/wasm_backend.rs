use crate::backend::Backend;

pub struct WasmBackend {

}

impl WasmBackend {
    pub fn new() -> Self {
        Self {}
    }
}

extern "C" {
    fn _log(ptr: *const u8, len: usize);
    fn _elog(ptr: *const u8, len: usize);
}

impl Backend for WasmBackend {
    fn draw_line(&mut self, a: (i32, i32), b: (i32, i32)) {
        todo!()
    }

    fn set_draw_color(&mut self, color: crate::backend::Color) {
        todo!()
    }

    fn clear(&mut self) {
        todo!()
    }

    fn win_size(&self) -> (u32, u32) {
        todo!()
    }

    fn poll_event(&mut self) -> Vec<crate::backend::Event> {
        todo!()
    }

    fn draw_point(&mut self, x: i32, y: i32) {
        todo!()
    }

    fn text_size(&self, string: &str) -> (u32, u32) {
        todo!()
    }

    fn render_text(&mut self, text: &str, x: i32, y: i32, color: crate::backend::Color) {
        todo!()
    }

    fn render_png_data(&mut self, data: &[u8], x: i32, y: i32, w: i32, h: i32) {
        todo!()
    }

    fn present(&mut self) {
        todo!()
    }

    fn log(string: String) {
        unsafe {_log(string.as_ptr(), string.bytes().count())};
    }

    fn elog(string: String) {
        unsafe {_elog(string.as_ptr(), string.bytes().count())};
    }
}

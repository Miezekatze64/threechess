use std::io::Read;
use crate::backend::{self, Color, Event, MouseButton};

use sdl2::{render::{Canvas, TextureCreator}, video::{Window, WindowContext}, EventPump, ttf::{Font, Sdl2TtfContext}, rect::Rect};
use sdl2::image::LoadTexture;

pub struct SdlBackend<'a, 'b> {
    canvas: Canvas<Window>,
    event_pump: EventPump,
    texture_creator: TextureCreator<WindowContext>,
    ttf: TTFWrapper<'a, 'b>,
}

struct TTFWrapper<'a, 'b> {
    font: Option<Font<'a, 'b>>,
    ctx: Sdl2TtfContext,
}

impl<'a, 'b: 'a> TTFWrapper<'a, 'b> {
    fn new() -> Self {
        Self {
            font: None,
            ctx: sdl2::ttf::init().unwrap(),
        }
    }

    fn add_font(&mut self) {
        self.font = unsafe {
            std::mem::transmute::<_, Option<Font<'static, 'static>>>(
                self.ctx.load_font("./FiraCode.ttf", 18).ok()
            )
        };
    }

    fn font(&'a self) -> &'a Font {
        self.font.as_ref().unwrap()
    }
}

impl<'a, 'b: 'a> SdlBackend<'a, 'b> {
    pub fn new() -> Self {
        let ctx = sdl2::init().unwrap();
        let vid = ctx.video().unwrap();

        let win = vid.window("threechess", 800, 800)
                     .resizable()
                     .build()
                     .unwrap();

        let canvas = win.into_canvas()
                        .accelerated()
                        .present_vsync()
                        .build()
                        .unwrap();
        let event_pump = ctx.event_pump().unwrap();
        let texture_creator = canvas.texture_creator();

        sdl2::image::init(sdl2::image::InitFlag::PNG).unwrap();

        let ttf = TTFWrapper::new();
        let mut obj = Self {
            canvas,
            event_pump,
            texture_creator,
            ttf,
        };

        obj.ttf.add_font();
        obj
    }

}

fn color_to_sdl_color(color: Color) -> sdl2::pixels::Color {
    sdl2::pixels::Color::RGB(color.0, color.1, color.2)
}

impl<'a, 'b> backend::Backend for SdlBackend<'a, 'b> {
    fn draw_line(&mut self, a: (i32, i32), b: (i32, i32)) {
        self.canvas.draw_line(a, b).unwrap();
    }

    fn set_draw_color(&mut self, color: backend::Color) {
        self.canvas.set_draw_color(color_to_sdl_color(color));
    }

    fn clear(&mut self) {
        self.canvas.clear();
    }

    fn win_size(&self) -> (u32, u32) {
        self.canvas.window().size()
    }

    fn poll_event(&mut self) -> Vec<Event> {
        self.event_pump.poll_iter()
            .flat_map(|x| {
                match x {
                    sdl2::event::Event::Quit { .. } => Some(Event::Quit),
                    sdl2::event::Event::MouseButtonUp { mouse_btn, x, y, .. } =>
                        Some(Event::MouseButtonUp(match mouse_btn {
                            sdl2::mouse::MouseButton::Left => MouseButton::Left,
                            _ => MouseButton::Unknown,
                        }, x, y)),
                    _ => None,
                }
            }).collect()
    }

    fn draw_point(&mut self, x: i32, y: i32) {
        self.canvas.draw_point((x, y)).unwrap();
    }

    fn text_size(&self, string: &str) -> (u32, u32) {
        self.ttf.font().size_of(string).unwrap()
    }

    fn render_text(&mut self, text: &str, x: i32, y: i32, color: Color) {
        let surf = self.ttf.font().render(text)
                       .solid(color_to_sdl_color(color)).unwrap();

        let texture = self.texture_creator.create_texture_from_surface(surf).unwrap();

        let (tw, th) = self.ttf.font().size_of(text).unwrap();
        let target = Rect::new(x, y, tw, th);
        self.canvas.copy(&texture, None, Some(target)).unwrap();
    }

    fn render_png_data(&mut self, data: &[u8], x: i32, y: i32, w: i32, h: i32) {
        let text = self.texture_creator.load_texture_bytes(data).unwrap();
        let target = Rect::new(x, y, w as u32, h as u32);
        self.canvas.copy(&text, None, Some(target)).unwrap();
    }

    fn present(&mut self) {
        self.canvas.present();
    }

    fn log(string: String) {
        println!("{}", string);
    }

    fn elog(string: String) {
        eprintln!("{}", string);
    }

    fn read_file(&self, path: &str) -> Vec<u8> {
        let mut f = std::fs::File::open(path).unwrap();
        let mut data = vec![];
        f.read_to_end(&mut data).unwrap();
        data
    }
}

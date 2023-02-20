#![allow(unused_assignments)]

use std::io::Read;

use sdl2::{pixels::Color, rect::Rect, render::Canvas, image::LoadTexture};
#[derive(Default, Clone, Copy, Debug)]
pub struct Coord(char, usize);

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub enum FieldType {
    #[default] WHITE,
    BLACK,
}

#[derive(Clone, Copy, Debug)]
pub enum Player {
    Red = 0,
    Green = 1,
    Yellow = 2,
}

#[derive(Clone, Copy, Debug)]
pub enum PieceType {
    Pawn = 0,
    Rook,
    Knight,
    Bishop,
    Queen,
    King
}

pub const PIECE_LETTERS: [char; 6] = ['p','r','n','b','q','k'];
pub const COLOR_LETTERS: [char; 3] = ['r','g','y'];

#[derive(Clone, Copy, Debug)]
pub struct Piece {
    pub typ: PieceType,
    pub player: Player
}

#[derive(Default, Clone, Copy, Debug)]
pub struct Field {
    pub coord: Coord,
    pub typ: FieldType,
    pub piece: Option<Piece>,
}

impl Field {
    pub fn new(a: char, b: usize, tp: FieldType) -> Self {
        Self {
            coord: Coord(a, b),
            typ: tp,
            piece: None,
        }
    }
}

#[derive(Debug)]
pub struct Section {
    pub fields: [[Field; 4]; 4],
    pub start_file: char,
    pub start_rank: usize,
    points: fn (i32, i32) -> [(i32,i32); 4],
}

fn cadd(c: char, i: usize) -> char{
    (c as u8 + i as u8) as char
}

impl Section {
    pub fn new(f: char, r: usize, inverse_colors: bool,
               pts: fn(i32,i32)->[(i32,i32);4]) -> Self {
        let tp1 = if inverse_colors { FieldType::BLACK  }
        else { FieldType::WHITE };

        let tp2 = if ! inverse_colors { FieldType::BLACK  }
        else { FieldType::WHITE };

        Self {
            points: pts,
            fields: [[
                Field::new(f, r, tp2),
                Field::new(f, r+1, tp1),
                Field::new(f, r+2, tp2),
                Field::new(f, r+3, tp1),
            ],[
                Field::new(cadd(f, 1), r, tp1),
                Field::new(cadd(f, 1), r+1, tp2),
                Field::new(cadd(f, 1), r+2, tp1),
                Field::new(cadd(f, 1), r+3, tp2),
            ],[
                Field::new(cadd(f, 2), r, tp2),
                Field::new(cadd(f, 2), r+1, tp1),
                Field::new(cadd(f, 2), r+2, tp2),
                Field::new(cadd(f, 2), r+3, tp1),
            ],[
                Field::new(cadd(f, 3), r, tp1),
                Field::new(cadd(f, 3), r+1, tp2),
                Field::new(cadd(f, 3), r+2, tp1),
                Field::new(cadd(f, 3), r+3, tp2),
            ]],
            start_file: f,
            start_rank: r,
        }
    }
}

#[derive(Debug)]
struct Board {
    sections: [Section; 6],
    current_player: Player,
}

impl Board {
    pub fn new() -> Self {
        let s = [
            Section::new('a', 1, false,
            |r, h| [(r/2, 0), (r, 0), (r, h), (r/4, h/2)]),
            Section::new('e', 1, false,
            |r, h| [(r, 0), (r+r/2, 0), (r+3*r/4, h/2), (r, h)]),
            Section::new('e', 9, false,
            |r, h| [(r, h), (r+3*r/4, h/2), (r*2, h), (r+3*r/4, h+h/2)]),
            Section::new('i', 9, true,
            |r, h| [(r, h), (r, h*2),(r+r/2, h*2), (r+3*r/4, h+h/2)]),
            Section::new('i', 5, false,
            |r, h| [(r, h), (r, h*2), (r/2, h*2), (r/4, h+h/2)]),
            Section::new('a', 5, false,
            |r, h| [(r/4, h/2), (r, h), (r/4, h+h/2), (0, h)]),
        ];
        Board {
            sections: s,
            current_player: Player::Red,
        }
    }

    fn get_field(&mut self, file: char, rank: usize) -> Option<&mut Field> {
        for s in &mut self.sections {
            for r in &mut s.fields {
                for f in r.iter_mut() {
                    if f.coord.0 == file
                        && f.coord.1 == rank {
                            return Some(f);
                        }
                }
            }
        }

        None
    }

    fn place_pieces_half(&mut self, rank: usize,
                         start_file: char, mut invf: bool, invr: bool,
                         right: bool, player: Player) {

        if right {
            invf ^= true;
        }

        self.get_field(cadd(start_file,
                            if invf {3} else {0}), rank).unwrap()
                                .piece = Some(Piece {
                                    typ: PieceType::Rook,
                                    player,
                                });
        self.get_field(cadd(start_file, if invf {2} else {1}), rank).unwrap()
                                .piece = Some(Piece {
                                    typ: PieceType::Knight,
                                    player,
                                });
        self.get_field(cadd(start_file, if invf {1} else {2}), rank).unwrap()
                                .piece = Some(Piece {
                                    typ: PieceType::Bishop,
                                    player,
                                });
        self.get_field(cadd(start_file, if invf {0} else {3}), rank).unwrap()
                                .piece = Some(Piece {
                                    typ: if right {PieceType::King} else {PieceType::Queen},
                                    player,
                                });

        for a in 0 .. 4 {
            self.get_field(cadd(start_file,
                a), if invr {rank-1} else {rank+1}) .unwrap()
                        .piece = Some(Piece {
                            typ: PieceType::Pawn,
                            player,
                        });
        }
    }

    pub fn place_pieces(&mut self) {
        self.place_pieces_half( 1, 'a', false, false, false, Player::Red);
        self.place_pieces_half( 1, 'e', false, false, true,  Player::Red);

        self.place_pieces_half( 8, 'i', true,  true, false, Player::Green);
        self.place_pieces_half( 8, 'a', true,  true, true, Player::Green);

        self.place_pieces_half(12, 'e', true,  true, false, Player::Yellow);
        self.place_pieces_half(12, 'i', false,  true, true,  Player::Yellow);
    }
}

fn draw_polygon<T: sdl2::render::RenderTarget>
    (canvas: &mut Canvas<T>, points: &[(i32, i32)]) {
    canvas.draw_line(points[0], points[2]).unwrap();
    canvas.draw_line(points[2], points[3]).unwrap();
    canvas.draw_line(points[3], points[1]).unwrap();
    canvas.draw_line(points[1], points[0]).unwrap();
}

fn sign(p1: (i32, i32), p2: (i32, i32), p3: (i32, i32)) -> i32 {
    (p1.0 - p3.0) * (p2.1 - p3.1) - (p2.0 - p3.0) * (p1.1 - p3.1)
}

fn point_is_in_triangle(pt: (i32, i32), points: &[(i32, i32); 3]) -> bool{
    let d1 = sign(pt, points[0], points[1]);
    let d2 = sign(pt, points[1], points[2]);
    let d3 = sign(pt, points[2], points[0]);

    let has_neg = d1 < 0 || d2 < 0 || d3 < 0;
    let has_pos = d1 > 0 || d2 > 0 || d3 > 0;

    !(has_neg && has_pos)
}

fn point_is_in_quadrilateral(pt: (i32, i32), points: &[(i32, i32); 4]) -> bool {
    point_is_in_triangle(pt, &[points[0], points[1], points[2]])
        ||
    point_is_in_triangle(pt, &[points[1], points[2], points[3]])
}

fn fill_quadrilateral<T: sdl2::render::RenderTarget>
    (canvas: &mut Canvas<T>, points: &[(i32, i32); 4]) {
    let lx = points[0].0
        .min(points[1].0)
        .min(points[2].0)
        .min(points[3].0);

    let rx = points[0].0
        .max(points[1].0)
        .max(points[2].0)
        .max(points[3].0);

    let ly = points[0].1
        .min(points[1].1)
        .min(points[2].1)
        .min(points[3].1);

    let ry = points[0].1
        .max(points[1].1)
        .max(points[2].1)
        .max(points[3].1);

    for y in ly ..= ry {
        for x in lx ..= rx {
            if point_is_in_quadrilateral((x, y), points) {
                canvas.draw_point((x, y)).unwrap();
            }
        }
    }
}


fn main_loop(board: Board, textures: Vec<Vec<Image>>) {
    let ctx = sdl2::init().unwrap();
    let vid = ctx.video().unwrap();

    let win = vid.window("threechess", 800, 800)
        .resizable()
        .build()
        .unwrap();

    let mut canvas = win.into_canvas()
                    .accelerated()
                    .present_vsync()
                    .build()
                    .unwrap();
    let mut event_pump = ctx.event_pump().unwrap();

    let ttf = sdl2::ttf::init().unwrap();
    let font = ttf.load_font("./FiraCode.ttf", 13).unwrap();

    let texture_creator = canvas.texture_creator();

    sdl2::image::init(sdl2::image::InitFlag::PNG).unwrap();

    'running: loop {
        canvas.set_draw_color(Color::WHITE);
        canvas.clear();

        for e in event_pump.poll_iter() {
            if let sdl2::event::Event::Quit { .. } = e {
                break 'running;
            }
        }

        let mut ww = canvas.window().size().0 as i32;
        let mut wh = canvas.window().size().1 as i32;

        let radius: i32 = ((ww.min(wh) / 2) as f32 * 0.9) as _;
        let height: i32 = ((3f32.sqrt() * radius as f32) / 2.0) as i32;

        let offx = (ww - 2 * radius) / 2;
        let offy = (wh - 2 * height) / 2;

        ww -= offx;
        wh -= offy;

        for s in &board.sections {
            let pts = (s.points)(radius, height);

            let y0 = wh - pts[0].1;
            let y1 = wh - pts[1].1;
            let y2 = wh - pts[2].1;
            let y3 = wh - pts[3].1;

            let x0 = offx + pts[0].0;
            let x1 = offx + pts[1].0;
            let x2 = offx + pts[2].0;
            let x3 = offx + pts[3].0;

            for y in 0 .. 4 {
                for x in 0 .. 4 {
                    let mut points = [(0, 0); 4];

                    let coords = s.get_coords(x, y);

                    for a in 0 .. 2 {
                        for b in 0 .. 2 {
                            let xfrac = (x + a) as f32 / 4.0;
                            let yfrac = (y + b) as f32 / 4.0;

                            let xo0 = x0 as f32 + (x1 - x0) as f32 * xfrac;
                            let xo1 = x3 as f32 + (x2 - x3) as f32 * xfrac;
                            let rx = xo0 + (xo1 - xo0) * yfrac;

                            let yo0 = y0 as f32 + (y3 - y0) as f32 * yfrac;
                            let yo1 = y1 as f32 + (y2 - y1) as f32 * yfrac;
                            let ry = yo0 + (yo1 - yo0) * xfrac;

                            points[2 * a as usize + b as usize] = (rx as i32, ry as i32);
                        }
                    }

                    let f = s.fields[x][y];

                    if f.typ == FieldType::BLACK {
                        canvas.set_draw_color(Color::BLACK);
                        fill_quadrilateral(&mut canvas, &points);
                    }
                    canvas.set_draw_color(Color::RED);
                    draw_polygon(&mut canvas, &points);

                    let mx = (points[0].0 + points[1].0 + points[2].0 + points[3].0) / 4;
                    let my = (points[0].1 + points[1].1 + points[2].1 + points[3].1) / 4;

                    let st = format!("{}{}", ((s.start_file as u8 + x as u8) as char)
                                     .to_uppercase(),
                                     s.start_rank as i32 + y as i32);

                    let surf = font.render(&st)
                        .solid(if f.typ == FieldType::BLACK {
                            Color::RGB(0xdd, 0xdd, 0xdd)
                        } else {
                            Color::RGB(0x22, 0x22, 0x22)
                        }).unwrap();

                    let text = texture_creator.create_texture_from_surface(surf).unwrap();

                    let (w, h) = font.size_of(&st).unwrap();
                    let target = Rect::new(mx - w as i32 / 2, my - h as i32 / 2, w, h);
                    canvas.copy(&text, None, Some(target)).unwrap();

                    if let Some(p) = f.piece {
                        let color = p.player as usize;
                        let piece = p.typ as usize;

                        let texture = &textures[color][piece];
                        let text = texture_creator.load_texture_bytes(&texture.data).unwrap();

                        let w = height / 6;
                        let h = w;

                        let target = Rect::new(mx - w / 2, my - h / 2, w as u32, h as u32);
                        canvas.copy(&text, None, Some(target)).unwrap();
                    }
                }
            }
        }
        canvas.present();
    }
}

#[derive(Debug)]
pub struct Image {
    pub data: Vec<u8>,
}

fn load_textures() -> std::io::Result<Vec<Vec<Image>>> {
    let mut vec = vec![];
    for c in COLOR_LETTERS {
        let mut inner_vec = vec![];
        for p in PIECE_LETTERS {
            let mut f = std::fs::File::open(&format!("./assets/{p}{c}.png"))?;
            let mut data = vec![];
            f.read_to_end(&mut data)?;

            inner_vec.push(Image {
                data,
            });
        }
        vec.push(inner_vec);
    }
    Ok(vec)
}

fn main() {
    let mut board = Board::new();
    board.place_pieces();
    let textures = load_textures().unwrap();

//    println!("Board: {board:#?}");
    main_loop(board, textures);
}

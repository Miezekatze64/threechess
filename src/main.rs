use sdl2::{pixels::Color, rect::Point};

#[derive(Default, Clone, Copy, Debug)]
struct Coord(char, usize);

#[derive(Default, Clone, Copy, Debug)]
struct Field {
    coord: Coord,
}

impl Field {
    pub fn new(a: char, b: usize) -> Self {
        Self {
            coord: Coord(a, b),
        }
    }
}

#[derive(Debug)]
struct Section {
    fields: [[Field; 4]; 4],
    start_file: char,
    start_rank: usize,
}

fn cadd(c: char, i: usize) -> char{
    (c as u8 + i as u8) as char
}

impl Section {
    pub fn new(f: char, r: usize) -> Self {
        Self {
            fields: [[
                Field::new(f, r),
                Field::new(cadd(f, 1), r),
                Field::new(cadd(f, 2), r),
                Field::new(cadd(f, 3), r),
            ],[
                Field::new(f, r+1),
                Field::new(cadd(f, 1), r+1),
                Field::new(cadd(f, 2), r+1),
                Field::new(cadd(f, 3), r+1),
            ],[
                Field::new(f, r+1),
                Field::new(cadd(f, 1), r+2),
                Field::new(cadd(f, 2), r+2),
                Field::new(cadd(f, 3), r+2),
            ],[
                Field::new(f, r+1),
                Field::new(cadd(f, 1), r+3),
                Field::new(cadd(f, 2), r+3),
                Field::new(cadd(f, 3), r+3),
            ]],
            start_file: f,
            start_rank: r,
        }
    }
}

#[derive(Debug)]
struct Board {
    sections: [Section; 6]
}

impl Board {
    pub fn new() -> Self {
        let s = [
            Section::new('a', 1),
            Section::new('e', 1),
            Section::new('e', 9),
            Section::new('i', 9),
            Section::new('i', 5),
            Section::new('a', 5),
        ];
        Board {
            sections: s,
        }
    }
}

fn main_loop(board: Board) {
    let ctx = sdl2::init().unwrap();
    let vid = ctx.video().unwrap();

    let win = vid.window("threechess", 800, 800)
        .resizable()
        .build()
        .unwrap();

    let mut canvas = win.into_canvas()
                    .accelerated()
                    .build()
                    .unwrap();
    let mut event_pump = ctx.event_pump().unwrap();

    'running: loop {
        canvas.set_draw_color(Color::WHITE);
        canvas.clear();

        for e in event_pump.poll_iter() {
            match e {
                sdl2::event::Event::Quit { .. } => {
                    break 'running;
                },
                _ => {}
            }
        }

        const RADIUS: i32 = 200;
        const HEIGHT: i32 = (3 * RADIUS) / 2;

//        for s in &board.sections {

        let mut ww = canvas.window().size().0 as i32;
        let mut wh = canvas.window().size().1 as i32;
        let offx = (ww - 2 * RADIUS) / 2;
        let offy = (wh - 2 * RADIUS) / 2;

        println!("OFF: ({offx}, {offy})");

        ww -= offx;
        wh -= offy;

        let y0 = wh;
        let y1 = wh;
        let y2 = wh - HEIGHT;
        let y3 = wh - HEIGHT / 2;

        let x0 = offx + RADIUS / 2;
        let x1 = offx + RADIUS;
        let x2 = offx + RADIUS;
        let x3 = offx + RADIUS / 4;

        let s = &board.sections[0];
        canvas.set_draw_color(Color::BLACK);
        for y in 0 .. 4 {
            for x in 0 .. 4 {
                let mut points = vec![];

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

                        points.push((rx as i32, ry as i32));
                    }
                }

                canvas.draw_line(points[0], points[2]).unwrap();
                canvas.draw_line(points[2], points[3]).unwrap();
                canvas.draw_line(points[3], points[1]).unwrap();
                canvas.draw_line(points[1], points[0]).unwrap();
            }
        }


        // canvas.draw_line((x0, y0), (x1, y1)).unwrap();
        // canvas.draw_line((x1, y1), (x2, y2)).unwrap();
        // canvas.draw_line((x2, y2), (x3, y3)).unwrap();
        // canvas.draw_line((x3, y3), (x0, y0)).unwrap();
//        }

        canvas.present();
    }
}

fn main() {
    let board = Board::new();
    println!("Board: {board:#?}");
    main_loop(board);
}

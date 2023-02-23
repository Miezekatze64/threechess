#![allow(unused)]

use std::io::Read;

use sdl2::{pixels::Color, rect::Rect, render::Canvas, image::LoadTexture, mouse::MouseButton};
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub struct Coord(char, usize);

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub enum FieldType {
    #[default] WHITE,
    BLACK,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Player {
    Red = 0,
    Green = 1,
    Yellow = 2,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Direction {
    ForwardRed,
    ForwardYellow,
    ForwardGreen,
    RedRight,
    RedLeft,
    GreenRight,
    GreenLeft,
    YellowRight,
    YellowLeft,

    // diagonals
    RedYellowToRed,
    RedYellowToYellow,
    GreenRedToRed,
    GreenRedToGreen,
    GreenYellowToYellow,
    GreenYellowToGreen,
    RedToRedYellow,
    RedToGreenRed,
    YellowToRedYellow,
    YellowToGreenYellow,
    GreenToGreenRed,
    GreenToGreenYellow,
}

impl Direction {
    pub fn is_straight(&self) -> bool {
        self <= &Direction::YellowLeft
    }

    pub fn all() -> Vec<Self> {
        let mut v = vec![];
        for a in (Self::ForwardRed as usize
                  ..= Self::GreenToGreenYellow as usize) {
            let var: Self = unsafe {std::mem::transmute(a as u8)};
            v.push(var);
        }
        v
    }

    pub fn is_opposite(&self, other: &Self) -> bool {
        match self {
            Self::ForwardRed => other == &Self::ForwardGreen || other == &Self::ForwardYellow,
            Self::ForwardYellow => other == &Self::ForwardGreen || other == &Self::ForwardRed,
            Self::ForwardGreen => other == &Self::ForwardRed || other == &Self::ForwardYellow,
            Self::RedRight => other == &Self::RedLeft,
            Self::RedLeft => other == &Self::RedRight,
            Self::GreenRight => other == &Self::GreenLeft,
            Self::GreenLeft => other == &Self::GreenRight,
            Self::YellowRight => other == &Self::YellowLeft,
            Self::YellowLeft => other == &Self::YellowRight,
            _ => false,
        }
    }

    pub fn orthogonals(&self) -> Vec<Self> {
        Self::all().into_iter().filter(|x| x != self && x.is_straight() && ! self.is_opposite(x))
                          .collect()
    }

    pub fn next(&self, start: &Field, board: &Board, player: &Player) -> Option<Field> {
        let (f, r) = match self {
            Direction::ForwardRed => {
                if start.coord.0 >= 'i' {
                    return None;
                }

                if start.coord.1 == 8 || start.coord.1 == 12 {
                    return None;
                }

                let r = if start.coord.1 == 4
                    && start.coord.0 >= 'e' {
                        9
                    } else {
                        start.coord.1 + 1
                    };

                (start.coord.0, r)
            },
            Direction::ForwardYellow => {
                if start.coord.0 <= 'd' {
                    return None;
                }

                if start.coord.1 == 8 || start.coord.1 == 1 {
                    return None;
                }

                let r = if start.coord.1 == 9 {
                    if start.coord.0 >= 'i' {
                        5
                    } else {
                        4
                    }
                } else if start.coord.1 >= 9 {
                    start.coord.1 - 1
                } else if start.coord.1 >= 5 {
                    start.coord.1 + 1
                } else {
                    start.coord.1 - 1
                };

                (start.coord.0, r)
            },
            Direction::ForwardGreen => {
                if start.coord.0 >= 'e'
                && start.coord.0 <= 'h' {
                    return None;
                }

                if start.coord.1 == 12 || start.coord.1 == 1 {
                    return None;
                }

                let r = if start.coord.1 == 5
                && start.coord.0 >= 'i' {
                    9
                } else if start.coord.1 >= 9 {
                    start.coord.1 + 1
                } else {
                    start.coord.1 - 1
                };

                (start.coord.0, r)
            },
            Direction::RedRight => {
                if start.coord.1 >= 5 {
                    return None;
                }
                let f = cadd(start.coord.0, 1);

                (f, start.coord.1)
            },
            Direction::RedLeft => {
                if start.coord.1 >= 5 {
                    return None;
                }
                let f = csub(start.coord.0, 1);

                (f, start.coord.1)
            },
            Direction::GreenRight => {
                if start.coord.1 <= 4 {
                    return None;
                }
                if start.coord.1 >= 9 {
                    return None;
                }
                let f = if start.coord.0 == 'i' {
                    'd'
                } else {
                    csub(start.coord.0, 1)
                };

                (f, start.coord.1)
            },
            Direction::GreenLeft => {
                if start.coord.1 <= 4 {
                    return None;
                }
                if start.coord.1 >= 9 {
                    return None;
                }
                let f = if start.coord.0 == 'd' {
                    'i'
                } else {
                    cadd(start.coord.0, 1)
                };

                (f, start.coord.1)
            },
            Direction::YellowRight => {
                if start.coord.1 <= 8 {
                    return None;
                }
                let f = if start.coord.0 == 'e' {
                    'i'
                } else if start.coord.0 >= 'i' {
                    cadd(start.coord.0, 1)
                } else {
                    csub(start.coord.0, 1)
                };

                (f, start.coord.1)
            },
            Direction::YellowLeft => {
                if start.coord.1 <= 8 {
                    return None;
                }

                if start.coord.0 == 'h' {
                    return None;
                }

                let f = match start.coord.0 {
                    'i' => 'e',
                    x if x > 'i'=> csub(x, 1),
                    a => cadd(a, 1)
                };

                (f, start.coord.1)
            },
            Direction::RedYellowToRed => {
                if start.coord.0 >= 'i' ||
                (start.coord.1 >= 5 && start.coord.1 <= 8) {
                    return None;
                }

                if start.coord.1 == 1 {
                    return None;
                }

                if start.coord.1 >= 9 && (start.coord.1 -
                    (start.coord.0 as u8 - b'a' + 1) as usize
                    >= 5) {
                    return None;
                }

                if start.coord.1 <= 8 && start.coord.1 >
                    (start.coord.0 as u8 - b'a' + 1) as usize {
                    return None;
                }

                let f = csub(start.coord.0, 1);
                let r = match start.coord.1 {
                    9 => 4,
                    x => x-1,
                };

                (f, r)
            },
            Direction::RedYellowToYellow => {
                if start.coord.0 <= 'd' ||
                    (start.coord.1 >= 5 && start.coord.1 <= 8) {
                    return None;
                }

                if start.coord.1 == 12 {
                    return None;
                }

                if start.coord.1 >= 9 && start.coord.1 <
                    (start.coord.0 as u8 - b'a' + 1) as usize {
                    return None;
                }

                if start.coord.1 <= 4 && (start.coord.1 +
                    (start.coord.0 as u8 - b'a' + 1) as usize
                    <= 8) {
                    return None;
                }

                let f = match start.coord.0 {
                    'e' => 'i',
                    x if x <= 'h' => csub(start.coord.0, 1),
                    x  => cadd(start.coord.0, 1),
                };
                let r = match start.coord.1 {
                    4 => 9,
                    x => x+1,
                };

                (f, r)
            },
            Direction::GreenRedToRed => {
                if start.coord.0 >= 'i' {
                    return None;
                }

                if start.coord.1 == 1 {
                    return None;
                }

                if start.coord.1 +
                    (start.coord.0 as u8 - b'a' + 1) as usize >= 10{
                    return None;
                }

                let f = cadd(start.coord.0, 1);
                let r = start.coord.1 - 1;
                (f, r)
            },
            Direction::GreenRedToGreen => {
                if start.coord.1 >= 9 {
                    return None;
                }

                if start.coord.1 == 8 {
                    return None;
                }

                if start.coord.0 <= 'h' && (start.coord.1 <
                    (start.coord.0 as u8 - b'a' + 1) as usize) {
                    return None;
                }

                if start.coord.0 >= 'i'
                    && ((start.coord.0 as u8 - b'a' + 1) as usize
                        - start.coord.1) >= 5 {
                    return None;
                }

                let f = match start.coord.0 {
                    'd' => 'i',
                    x => cadd(x, 1),
                };
                let r = start.coord.1 + 1;
                (f, r)
            },
            Direction::GreenYellowToYellow => {
                if start.coord.1 <= 4 || start.coord.0 <= 'd' {
                    return None;
                }

                if start.coord.1 == 12 {
                    return None;
                }

                if start.coord.0 <= 'h' && (start.coord.1 -
                    (start.coord.0 as u8 - b'a' + 1) as usize) <= 3 {
                    return None;
                }

                if start.coord.1 <= 8
                    && ((start.coord.0 as u8 - b'a' + 1) as usize - start.coord.1
                    ) <= 3 {
                    return None;
                }

                let f = match start.coord.0 {
                    'i' => 'e',
                    x if x >= 'i' => csub(x, 1),
                    x  => cadd(x, 1),
                };
                let r = match start.coord.1 {
                    5 => 9,
                    x if x <= 8 => x - 1,
                    x  => x + 1,
                };

                (f, r)
            },
            Direction::GreenYellowToGreen => {
                if start.coord.1 <= 4 || (start.coord.0 >= 'e'
                                          && start.coord.0 <= 'h') {
                    return None;
                }

                if start.coord.1 == 8 {
                    return None;
                }

                if start.coord.0 <= 'd' && (start.coord.1 +
                    (start.coord.0 as u8 - b'a' + 1) as usize) <= 8 {
                    return None;
                }

                if start.coord.0 >= 'i'
                    && ((start.coord.0 as u8 - b'a' + 1) as usize) < start.coord.1 {
                    return None;
                }

                let f = match start.coord.0 {
                    'i' => 'd',
                    x  => csub(x, 1),
                };
                let r = match start.coord.1 {
                    9 => 5,
                    x if x > 9 => x - 1,
                    x => x + 1,
                };

                (f, r)
            },
            Direction::RedToRedYellow => {
                if start.coord.0 >= 'i' || (start.coord.1 >= 5
                                          && start.coord.1 <= 8) {
                    return None;
                }

                if start.coord.0 == 'h' {
                    return None;
                }

                if start.coord.0 <= 'd' && (start.coord.1 >
                    (start.coord.0 as u8 - b'a' + 1) as usize) {
                    return None;
                }

                if start.coord.1 >= 9
                    && (start.coord.1 - (start.coord.0 as u8 - b'a' + 1) as usize) >= 5 {
                    return None;
                }

                let f = cadd(start.coord.0, 1);
                let r = match start.coord.1 {
                    4 => 9,
                    x => x + 1,
                };

                (f, r)
            },
            Direction::RedToGreenRed => {
                if start.coord.1 >= 9 || start.coord.0 >= 'i' {
                    return None;
                }

                if start.coord.0 == 'a' {
                    return None;
                }

                if start.coord.1 >= 5 && (start.coord.1 +
                    (start.coord.0 as u8 - b'a' + 1) as usize) >= 10 {
                    return None;
                }

                if start.coord.0 >= 'e'
                    && (start.coord.1 + (start.coord.0 as u8 - b'a' + 1) as usize) >= 10 {
                    return None;
                }

                let f = csub(start.coord.0, 1);
                let r = start.coord.1 + 1;

                (f, r)
            },
            Direction::YellowToRedYellow => {
                if (start.coord.1 <= 8
                    && start.coord.1 >= 5) || start.coord.0 <= 'd' {
                    return None;
                }

                if start.coord.0 == 'h' {
                    return None;
                }

                if start.coord.1 <= 4 && (start.coord.1 +
                    (start.coord.0 as u8 - b'a' + 1) as usize) <= 8 {
                    return None;
                }

                if start.coord.0 >= 'i'
                    && start.coord.1 < (start.coord.0 as u8 - b'a' + 1) as usize {
                    return None;
                }

                let f = match start.coord.0 {
                    'i' => 'e',
                    x if x >= 'i' => csub(x, 1),
                    x => cadd(x, 1),
                };
                let r = match start.coord.1 {
                    9 => 4,
                    x => x - 1,
                };

                (f, r)
            },
            Direction::YellowToGreenYellow => {
                if start.coord.0 <= 'd'
                    || start.coord.1 <= 4 {
                    return None;
                }

                if start.coord.0 == 'l' {
                    return None;
                }

                if start.coord.0 <= 'h' && (start.coord.1 -
                    (start.coord.0 as u8 - b'a' + 1) as usize) <= 3 {
                    return None;
                }

                if start.coord.1 <= 8
                    && ((start.coord.0 as u8 - b'a' + 1) as usize - start.coord.1) <= 3 {
                    return None;
                }

                let f = match start.coord.0 {
                    'e' => 'i',
                    x if x >= 'i' => cadd(x, 1),
                    x => csub(x, 1),
                };
                let r = match start.coord.1 {
                    9 => 5,
                    x if x >= 9 => x - 1,
                    x => x + 1,
                };

                (f, r)
            },
            Direction::GreenToGreenRed => {
                if start.coord.1 >= 9 || (
                    start.coord.0 <= 'h' &&  start.coord.0 >= 'e') {
                    return None;
                }

                if start.coord.0 == 'a' {
                    return None;
                }

                if start.coord.1 <= 4 && start.coord.1 <
                    (start.coord.0 as u8 - b'a' + 1) as usize {
                    return None;
                }

                if start.coord.0 >= 'i'
                    && ((start.coord.0 as u8 - b'a' + 1) as usize
                    - start.coord.1) >= 5 {
                    return None;
                }

                let f = match start.coord.0 {
                    'i' => 'd',
                    x => csub(x, 1),
                };
                let r = start.coord.1 - 1;
                (f, r)
            },
            Direction::GreenToGreenYellow => {
                if (start.coord.0 <= 'h' && start.coord.0 >= 'e')
                    || start.coord.1 <= 4 {
                    return None;
                }

                if start.coord.0 == 'l' {
                    return None;
                }

                if start.coord.0 <= 'd' && (start.coord.1 +
                    (start.coord.0 as u8 - b'a' + 1) as usize) <= 8 {
                    return None;
                }

                if start.coord.1 >= 9
                    && ((start.coord.0 as u8 - b'a' + 1) as usize)
                        < start.coord.1 {
                    return None;
                }

                let f = match start.coord.0 {
                    'd' => 'i',
                    x => cadd(x, 1),
                };
                let r = match start.coord.1 {
                    5 => 9,
                    x if x >= 9 => x + 1,
                    x => x - 1,
                };

                (f, r)
            },
        };

        board.get_field(f, r).cloned()
    }
}

impl Field {
    pub fn new(a: char, b: usize, tp: FieldType) -> Self {
        Self {
            coord: Coord(a, b),
            typ: tp,
            piece: None,
        }
    }

    fn get_pawn_dirs(&self, player: Player) -> Vec<Direction> {
        vec![match player {
            Player::Red => {
                if (self.coord.0 >= 'e' && self.coord.0 <= 'h') ||
                    (self.coord.0 >= 'a' && self.coord.0 <= 'd') {
                    Direction::ForwardRed
                } else if self.coord.1 <= 8 {
                    Direction::ForwardYellow
                } else {
                    Direction::ForwardGreen
                }
            },
            Player::Green => {
                if (self.coord.0 >= 'a' && self.coord.0 <= 'd') ||
                    (self.coord.0 >= 'i') {
                    Direction::ForwardGreen
                } else if self.coord.1 <= 4 {
                    Direction::ForwardYellow
                } else {
                    Direction::ForwardRed
                }
            },
            Player::Yellow => {
                if (self.coord.0 >= 'e' && self.coord.0 <= 'h') ||
                    (self.coord.0 >= 'i') {
                    Direction::ForwardYellow
                } else if self.coord.1 <= 4 {
                    Direction::ForwardGreen
                } else {
                    Direction::ForwardRed
                }
            },
        }]
    }

    fn get_possible_moves_unchecked(&self, board: &Board) -> Vec<Coord> {
        let piece = self.piece.unwrap();
        let player = piece.player;
        let fields = board.get_fields();

        let f = Board::get_fields(board);

        const STRAIGHT_DIRS: [Direction; 9] = [Direction::ForwardRed, Direction::ForwardYellow,
                                            Direction::ForwardGreen,
                                            Direction::RedRight, Direction::RedLeft,
                                            Direction::GreenRight, Direction::GreenLeft,
                                            Direction::YellowRight, Direction::YellowLeft,
        ];
        const DIAGONAL_DIRS: [Direction; 12] = [Direction::RedYellowToRed, Direction::RedYellowToYellow,
                                               Direction::GreenRedToRed, Direction::GreenRedToGreen,
                                               Direction::GreenYellowToYellow, Direction::GreenYellowToGreen,

                                               Direction::RedToRedYellow, Direction::RedToGreenRed,
                                               Direction::YellowToRedYellow, Direction::YellowToGreenYellow,
                                               Direction::GreenToGreenYellow, Direction::GreenToGreenRed,
        ];

        match piece.typ {
            PieceType::Pawn => {
                let move_dirs = self.get_pawn_dirs(player);
                // match player {
                //     Player::Red => vec![Direction::ForwardRed,
                //                         Direction::ForwardYellow,
                //                         Direction::ForwardGreen],
                //     Player::Green => vec![Direction::ForwardGreen],
                //     Player::Yellow => vec![Direction::ForwardYellow],
                // };

                let capture_dirs = match player {
                    Player::Red => [Direction::RedToGreenRed, Direction::RedToRedYellow,
                                    Direction::GreenYellowToGreen, Direction::GreenRedToGreen,
                                    Direction::GreenYellowToYellow, Direction::RedYellowToYellow],

                    Player::Green => [Direction::GreenToGreenRed, Direction::GreenToGreenYellow,
                                    Direction::RedYellowToRed, Direction::GreenRedToGreen,
                                    Direction::RedYellowToYellow, Direction::GreenYellowToYellow],

                    Player::Yellow => [Direction::YellowToGreenYellow, Direction::YellowToRedYellow,
                                    Direction::GreenRedToGreen, Direction::RedYellowToRed,
                                    Direction::GreenRedToRed, Direction::GreenYellowToGreen],
                };

                let is_at_home = match player {
                    Player::Red => self.coord.1 == 2,
                    Player::Green => self.coord.1 == 7,
                    Player::Yellow => self.coord.1 == 11,
                };

                let mut fields = vec![];
                for capture_dir in capture_dirs {
                    if let Some(a) = capture_dir.next(self, board, &player) {
                        if let Some(p) = a.piece {
                            if p.player != player {
                                fields.push(a.coord);
                            }
                        }
                    }
                }


                for move_dir in move_dirs {
                    if let Some(a) = move_dir.next(self, board, &player) {
                        if a.piece.is_none() {
                            fields.push(a.coord);

                            if is_at_home {
                                if let Some(a) = move_dir.next(&a, board, &player) {
                                    if a.piece.is_none() {
                                        fields.push(a.coord);
                                    }
                                }
                            }
                        }
                    }
                }

                fields
            },
            PieceType::Rook | PieceType::Bishop |
            PieceType::Queen => {
                let dirs = match piece.typ {
                    PieceType::Rook => STRAIGHT_DIRS.to_vec(),
                    PieceType::Queen => vec![STRAIGHT_DIRS.to_vec(),
                                              DIAGONAL_DIRS.to_vec()]
                        .into_iter().flatten().collect(),
                    PieceType::Bishop => DIAGONAL_DIRS.to_vec(),
                    _ => unreachable!("impossible"),
                };

                let mut fields = vec![];
                for direction in dirs {
                    let mut field = Some(*self);
                    while ({field = direction.next(&field.unwrap(), board, &player);
                            field.is_some()}) {
                        if field.is_none() {
                            break;
                        }

                        if let Some(p) = field.unwrap().piece {
                            if p.player != player {
                                fields.push(field.unwrap().coord);
                            }
                            break;
                        }

                        fields.push(field.unwrap().coord);
                    }
                }

                fields
            },
            PieceType::Knight => {
                let mut fields = vec![];

                // 1 - 2
                for dir in STRAIGHT_DIRS {
                    let f1 = dir.next(self, board, &player);
                    for orth in dir.orthogonals() {
                        let f3 = f1.and_then(|f| orth.next(&f, board, &player))
                            .and_then(|f| orth.next(&f, board, &player));
                        if let Some(f) = f3 {
                            if let Some(p) = f.piece {
                                if p.player == player {
                                    continue;
                                }
                            }
                            fields.push(f.coord);
                        }
                    }
                }

                // 2 - 1
                for dir in STRAIGHT_DIRS {
                    let f2 = dir.next(self, board, &player)
                        .and_then(|f| dir.next(&f, board, &player));
                    for orth in dir.orthogonals() {
                        let f3 = f2.and_then(|f| orth.next(&f, board, &player));
                        if let Some(f) = f3 {
                            if let Some(p) = f.piece {
                                if p.player == player {
                                    continue;
                                }
                            }
                            fields.push(f.coord);
                        }
                    }
                }
                fields
            }
            PieceType::King => {
                let mut dirs = STRAIGHT_DIRS.to_vec();
                dirs.append(&mut DIAGONAL_DIRS.to_vec());
                let mut fields = vec![];
                for d in dirs {
                    let field = d.next(self, board, &player);
                    if let Some(f) = field {
                        if let Some(p) = f.piece {
                            if p.player == player {
                                continue;
                            }
                        }
                        fields.push(f.coord);
                    }
                }
                fields
            },
        }
    }

    fn get_possible_moves(&self, board: &Board) -> Vec<Coord> {
        self.get_possible_moves_unchecked(board)
            .into_iter().filter(|x| {
                let mut new_board = board.clone();
                new_board.get_field_mut(x.0, x.1)
                    .unwrap().piece =
                    self.piece;

                new_board.get_field_mut(self.coord.0, self.coord.1)
                    .unwrap().piece =
                    None;

                ! new_board.is_check(self.piece.unwrap().player)
        }).collect()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Section {
    pub fields: [[Field; 4]; 4],
    pub start_file: char,
    pub start_rank: usize,
    points: fn (i32, i32) -> [(i32,i32); 4],
}

fn cadd(c: char, i: usize) -> char {
    (c as u8 + i as u8) as char
}

fn csub(c: char, i: usize) -> char {
    (c as u8 - i as u8) as char
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

    fn get_radius_and_height(&self, ww: i32, wh: i32) -> (i32, i32) {
        let radius: i32 = ((ww.min(wh) / 2) as f32 * 0.9) as _;
        let height: i32 = ((3f32.sqrt() * radius as f32) / 2.0) as i32;

        (radius, height)
    }

    fn get_coords(&self, x: usize, y: usize, mut ww: i32, mut wh: i32) -> [(i32, i32); 4] {
        let (radius, height) = self.get_radius_and_height(ww, wh);


        let offx = (ww - 2 * radius) / 2;
        let offy = (wh - 2 * height) / 2;

        ww -= offx;
        wh -= offy;

        let pts = (self.points)(radius, height);

        let y0 = wh - pts[0].1;
        let y1 = wh - pts[1].1;
        let y2 = wh - pts[2].1;
        let y3 = wh - pts[3].1;

        let x0 = offx + pts[0].0;
        let x1 = offx + pts[1].0;
        let x2 = offx + pts[2].0;
        let x3 = offx + pts[3].0;
        let mut coords = [(0, 0); 4];

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

                coords[2 * a as usize + b as usize] = (rx as i32, ry as i32);
            }
        }

        coords
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Board {
    sections: [Section; 6],
    current_player: Player,
    active_field: Option<Field>,
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
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
            active_field: None,
        }
    }

    pub fn get_fields(&self) -> Vec<Field> {
        self.sections.iter().flat_map(|x| x.fields
                         .into_iter()
                         .collect::<Vec<_>>()
                         ).flatten()
                          .collect::<Vec<_>>()
    }

    pub fn get_possible_move_targets_unchecked(&self, player: Player) -> Vec<Coord> {
        let fields = self.get_fields();
        let mut targets = vec![];
        for f in fields {
            if f.piece.is_none() {
                continue;
            }
            let mut moves = f.get_possible_moves_unchecked(self);
            targets.append(&mut moves);
        }
        targets
    }

    pub fn get_king_field(&self, player: Player) -> Field {
        let fields = self.get_fields();
        for f in fields {
            if let Some(p) = f.piece {
                if p.player == player && p.typ == PieceType::King {
                    return f;
                }
            }
        }
        unreachable!("the king of {player:?} died...?")
    }

    pub fn is_check(&self, player: Player) -> bool {
        let king = self.get_king_field(player);
        let moves = self.get_possible_move_targets_unchecked(player);

        moves.contains(&king.coord)
    }

    fn get_section_and_field(&self, file: char, rank: usize) -> Option<(Section, &Field)> {
        for s in &self.sections {
            let sect = *s;
            for r in &s.fields {
                for f in r.iter() {
                    if f.coord.0 == file
                        && f.coord.1 == rank {
                            return Some((sect, f));
                        }
                }
            }
        }

        None
    }

    fn get_section_and_field_mut(&mut self, file: char, rank: usize) -> Option<(Section, &mut Field)> {
        for s in &mut self.sections {
            let sect = *s;
            for r in &mut s.fields {
                for f in r.iter_mut() {
                    if f.coord.0 == file
                        && f.coord.1 == rank {
                            return Some((sect, f));
                        }
                }
            }
        }

        None
    }

    fn get_field(&self, file: char, rank: usize) -> Option<&Field> {
        self.get_section_and_field(file, rank).map(|(_, f)| f)
    }

    fn get_field_mut(&mut self, file: char, rank: usize) -> Option<&mut Field> {
        self.get_section_and_field_mut(file, rank).map(|(_, f)| f)
    }

    pub fn get_coords(&mut self, coord: Coord, ww: i32, wh: i32) -> Option<[(i32, i32); 4]> {
        let (sect, _) = self.get_section_and_field(coord.0, coord.1)?;

        for a in 0 .. 4 {
            for b in 0 .. 4 {
                if sect.fields[a][b].coord == coord {
                    return Some(sect.get_coords(a, b, ww, wh));
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

        self.get_field_mut(cadd(start_file,
                            if invf {3} else {0}), rank).unwrap()
                                .piece = Some(Piece {
                                    typ: PieceType::Rook,
                                    player,
                                });

        self.get_field_mut(cadd(start_file, if invf {0} else {3}), rank).unwrap()
                                .piece = Some(Piece {
                                    typ: if right {PieceType::King} else {PieceType::Queen},
                                    player,
                                });

        self.get_field_mut(cadd(start_file, if invf {2} else {1}), rank).unwrap()
                                .piece = Some(Piece {
                                    typ: PieceType::Knight,
                                    player,
                                });
        self.get_field_mut(cadd(start_file, if invf {1} else {2}), rank).unwrap()
                                .piece = Some(Piece {
                                    typ: PieceType::Bishop,
                                    player,
                                });

        for a in 0 .. 4 {
            self.get_field_mut(cadd(start_file,
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


fn main_loop(mut board: Board, textures: Vec<Vec<Image>>) {
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

        let ww = canvas.window().size().0 as i32;
        let wh = canvas.window().size().1 as i32;

        for e in event_pump.poll_iter() {
            match e {
                sdl2::event::Event::Quit { .. } => break 'running,
                sdl2::event::Event::MouseButtonUp { mouse_btn, x, y, .. } => {
                    if mouse_btn == MouseButton::Left {
                        let mut pressed_field = None;
                        'out: for s in &mut board.sections {
                            for yi in 0..4 {
                                for xi in 0..4 {
                                    let coords = s.get_coords(xi, yi, ww, wh);
                                    if point_is_in_quadrilateral((x, y), &coords) {
                                        pressed_field = Some(&mut s.fields[xi][yi]);
                                        break 'out;
                                    }
                                }
                            }
                        }

                        if let Some(f) = pressed_field {
                            if board.active_field.is_none() {
                                if let Some(p) = f.piece {
//                                    if p.player == board.current_player {
                                        board.active_field = Some(*f);
//                                    }
                                }
                            } else {
                                f.piece = board.active_field.unwrap().piece;
                                let af = board.active_field.unwrap().coord;
                                let mut_field = board.get_field_mut(af.0, af.1).unwrap();

                                mut_field.piece = None;
                                board.active_field = None;
                            }
                        }

                        println!("BUTTON {mouse_btn:?} at {x}/{y}");
                    }
                },
                _ => (),
            }
        }

        for s in &board.sections {
            for y in 0 .. 4 {
                for x in 0 .. 4 {
                    let points = s.get_coords(x, y, ww, wh);
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
                }
            }
        }

        if let Some(f) = board.active_field {
            let active_fields = f.get_possible_moves(&board);
            let coords: Vec<_> = active_fields.iter().flat_map(|f| board.get_coords(*f, ww, wh)).collect();

            for points in coords {
                canvas.set_draw_color(
                    [Color::RED, Color::GREEN, Color::YELLOW][board.current_player as usize]
                );
                let mx = (points[0].0 + points[1].0 + points[2].0 + points[3].0) / 4;
                let my = (points[0].1 + points[1].1 + points[2].1 + points[3].1) / 4;

                let r = ww.min(wh) / 40;

                for rx in (mx-r) ..= (mx+r) {
                    for ry in (my-r) ..= (my+r) {
                        let x = rx - mx;
                        let y = ry - my;

                        if x * x + y * y < r * r {
                            canvas.draw_point((rx, ry)).unwrap();
                        }
                    }
                }
            }
        }

        for s in board.sections {
            for y in 0 .. 4 {
                for x in 0 .. 4 {
                    let points = s.get_coords(x, y, ww, wh);
                    let f = s.fields[x][y];
                    let mx = (points[0].0 + points[1].0 + points[2].0 + points[3].0) / 4;
                    let my = (points[0].1 + points[1].1 + points[2].1 + points[3].1) / 4;

                    if let Some(p) = f.piece {
                        let color = p.player as usize;
                        let piece = p.typ as usize;

                        let texture = &textures[color][piece];
                        let text = texture_creator.load_texture_bytes(&texture.data).unwrap();

                        let (_, height) = s.get_radius_and_height(ww, wh);

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

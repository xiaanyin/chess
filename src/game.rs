use game::Player::Black;
use serde;
use serde_derive;
use std::cmp;
use std::collections::HashMap;
use std::panic::PanicInfo;

// 横向
const WIDTH: usize = 9;
// 纵向
const HEIGHT: usize = 10;
// 最大位置数
const MAX_CELLS_SIZE: usize = WIDTH * HEIGHT;
// 最大棋子数
const MAX_PIECES_SIZE: usize = 32;

// 帅 士 相 马 车 炮 兵
const RED_KING: char = 'K';
const RED_ADVISER: char = 'A';
const RED_BISHOP: char = 'B';
const RED_KNIGHT: char = 'N';
const RED_ROOK: char = 'R';
const RED_CANNON: char = 'C';
const RED_PAWN: char = 'P';
const BLACK_KING: char = 'k';
const BLACK_ADVISER: char = 'a';
const BLACK_BISHOP: char = 'b';
const BLACK_KNIGHT: char = 'n';
const BLACK_ROOK: char = 'r';
const BLACK_CANNON: char = 'c';
const BLACK_PAWN: char = 'p';

static INDEX_ROW: &'static [usize] = &[
    0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3,
    3, 3, 3, 3, 4, 4, 4, 4, 4, 4, 4, 4, 4, 5, 5, 5, 5, 5, 5, 5, 5, 5, 6, 6, 6, 6, 6, 6, 6, 6, 6, 7,
    7, 7, 7, 7, 7, 7, 7, 7, 8, 8, 8, 8, 8, 8, 8, 8, 8, 9, 9, 9, 9, 9, 9, 9, 9, 9,
];

// index of row end is [INDEX_ROW_START + 8], steps 1
static INDEX_ROW_START: &'static [usize] = &[
    0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 9, 9, 9, 9, 9, 9, 9, 9, 18, 18, 18, 18, 18, 18, 18, 18, 18, 27,
    27, 27, 27, 27, 27, 27, 27, 27, 36, 36, 36, 36, 36, 36, 36, 36, 36, 45, 45, 45, 45, 45, 45, 45,
    45, 45, 54, 54, 54, 54, 54, 54, 54, 54, 54, 63, 63, 63, 63, 63, 63, 63, 63, 63, 72, 72, 72, 72,
    72, 72, 72, 72, 72, 81, 81, 81, 81, 81, 81, 81, 81, 81,
];

static INDEX_COLUMN: &'static [usize] = &[
    0, 1, 2, 3, 4, 5, 6, 7, 8, 0, 1, 2, 3, 4, 5, 6, 7, 8, 0, 1, 2, 3, 4, 5, 6, 7, 8, 0, 1, 2, 3, 4,
    5, 6, 7, 8, 0, 1, 2, 3, 4, 5, 6, 7, 8, 0, 1, 2, 3, 4, 5, 6, 7, 8, 0, 1, 2, 3, 4, 5, 6, 7, 8, 0,
    1, 2, 3, 4, 5, 6, 7, 8, 0, 1, 2, 3, 4, 5, 6, 7, 8, 0, 1, 2, 3, 4, 5, 6, 7, 8,
];

// index of column end is [90 - INDEX_COLUMN_START], steps 9
static INDEX_COLUMN_START: &'static [usize] = &[
    0, 1, 2, 3, 4, 5, 6, 7, 8, 0, 1, 2, 3, 4, 5, 6, 7, 8, 0, 1, 2, 3, 4, 5, 6, 7, 8, 0, 1, 2, 3, 4,
    5, 6, 7, 8, 0, 1, 2, 3, 4, 5, 6, 7, 8, 0, 1, 2, 3, 4, 5, 6, 7, 8, 0, 1, 2, 3, 4, 5, 6, 7, 8, 0,
    1, 2, 3, 4, 5, 6, 7, 8, 0, 1, 2, 3, 4, 5, 6, 7, 8, 0, 1, 2, 3, 4, 5, 6, 7, 8,
];

// 帅 士 相 马 车 炮 兵
static EVALUATE_BASIC: &'static [i32] = &[0, 40, 40, 88, 200, 96, 9];

static EVALUATE_KING: &'static [i32] = &[
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, -9, -9, -9, 0, 0, 0, 0, 0, 0, -8, -8, -8, 0, 0, 0, 0, 0, 0, 1, 5, 1, 0, 0, 0,
];

static EVALUATE_ADVISER: &'static [i32] = &[
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];
static EVALUATE_BISHOP: &'static [i32] = &[
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    -2, 0, 0, 0, 3, 0, 0, 0, -2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];
static EVALUATE_KNIGHT: &'static [i32] = &[
    2, 2, 2, 8, 2, 8, 2, 2, 2, 2, 8, 15, 9, 6, 9, 15, 8, 2, 4, 10, 11, 15, 11, 15, 11, 10, 4, 5,
    20, 12, 19, 12, 19, 12, 20, 5, 2, 12, 11, 15, 16, 15, 11, 12, 2, 2, 10, 13, 14, 15, 14, 13, 10,
    2, 4, 6, 10, 7, 10, 7, 10, 6, 4, 5, 4, 6, 7, 4, 7, 6, 4, 5, -3, 2, 4, 5, -10, 5, 4, 2, -3, 0,
    -3, 2, 0, 2, 0, 2, -3, 0,
];
static EVALUATE_ROOK: &'static [i32] = &[
    6, 8, 7, 13, 14, 13, 7, 8, 6, 6, 12, 9, 16, 33, 16, 9, 12, 6, 6, 8, 7, 14, 16, 14, 7, 8, 6, 6,
    13, 13, 16, 16, 16, 13, 13, 6, 8, 11, 11, 14, 15, 14, 11, 11, 8, 8, 12, 12, 14, 15, 14, 12, 12,
    8, 4, 9, 4, 12, 14, 12, 4, 9, 4, -2, 8, 4, 12, 12, 12, 4, 8, -2, 5, 8, 6, 12, 0, 12, 6, 8, 5,
    -6, 6, 4, 12, 0, 12, 4, 6, -6,
];
static EVALUATE_CANNON: &'static [i32] = &[
    4, 4, 0, -5, -6, -5, 0, 4, 4, 2, 2, 0, -4, -7, -4, 0, 2, 2, 1, 1, 0, -5, -4, -5, 0, 1, 1, 0, 3,
    3, 2, 4, 2, 3, 3, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, -1, 0, 3, 0, 4, 0, 3, 0, -1, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 1, 0, 4, 3, 5, 3, 4, 0, 1, 0, 1, 2, 2, 2, 2, 2, 1, 0, 0, 0, 1, 3, 3, 3, 1, 0, 0,
];
static EVALUATE_PAWN: &'static [i32] = &[
    0, 0, 0, 2, 4, 2, 0, 0, 0, 20, 30, 50, 65, 70, 65, 50, 30, 20, 20, 30, 45, 55, 55, 55, 45, 30,
    20, 20, 27, 30, 40, 42, 40, 30, 27, 20, 10, 18, 22, 35, 40, 35, 22, 18, 10, 3, 0, 4, 0, 7, 0,
    4, 0, 3, -2, 0, -2, 0, 6, 0, -2, 0, -2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0,
];

// By default, player is Red, and computer is Black.
#[derive(Debug, Serialize, Deserialize)]
enum Player {
    Red,
    Black,
}

#[derive(Debug, Serialize, Deserialize)]
enum MinMax {
    Min,
    Max,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
struct MinMaxNode {
    piece: char,
    from: usize,
    to: usize,
}

struct Board {
    pieces_count: usize,
    positions: [Option<char>; MAX_CELLS_SIZE],
}

impl Board {
    pub fn new() -> Board {
        Board {
            pieces_count: 0usize,
            positions: [None; MAX_CELLS_SIZE],
        }
    }

    pub fn init_board(&mut self, fen: &str) {
        // rnbakabnr/9/1c5c1/p1p1p1p1p/9/9/P1P1P1P1P/1C5C1/9/RNBAKABNR
        self.clear();
        let mut position = 0usize;
        let mut pieces = 0usize;
        for c in fen.chars() {
            match c {
                '/' => continue,
                '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                    position += c.to_digit(10).unwrap() as usize;
                    continue;
                }
                _ => {
                    self.positions[position] = Some(c);
                    position += 1;
                    self.pieces_count += 1;
                }
            };
        }
    }

    pub fn search(&mut self) -> String {
        let search_depth: usize = match self.pieces_count {
            0..=4 => 6,
            5..=6 => 5,
            7..=16 => 4,
            17..=28 => 3,
            _ => 2,
        };

        let mut best_move: Option<MinMaxNode> = None;
        let mut best_value = i32::min_value();
        let mut all_moves: Vec<MinMaxNode> = self.generate_all_moves(&Player::Black);

        while let Some(node) = all_moves.pop() {
            let position_to_backup: Option<char> = self.temporary_move(node.from, node.to);
            let value = self.min_max(
                search_depth,
                i32::max_value(),
                i32::min_value(),
                &Player::Red,
            );
            if best_move.is_none() || value >= best_value {
                best_move = Some(node);
                best_value = value;
            }
            self.recovery(node.from, node.to, position_to_backup);
        }
        // TODO translate best_move to move instruction fen str
        // return best_move
        String::from("test")
    }

    fn min_max(&mut self, depth: usize, min: i32, max: i32, turn: &Player) -> i32 {
        match depth {
            0 => {
                // Or any of kings has been killed?
                self.evaluate(&turn)
            }
            _ => {
                let mut min: i32 = min;
                let mut max: i32 = max;
                let mut all_moves: Vec<MinMaxNode> = self.generate_all_moves(&turn);
                while let Some(node) = all_moves.pop() {
                    let position_to_backup: Option<char> = self.temporary_move(node.from, node.to);
                    match turn {
                        Player::Red => {
                            max = cmp::max(max, self.min_max(depth - 1, min, max, &Player::Black));
                        }
                        Player::Black => {
                            min = cmp::min(min, self.min_max(depth - 1, min, max, &Player::Red));
                        }
                    }
                    self.recovery(node.from, node.to, position_to_backup);
                    //    if min > max {
                    //        break;
                    //    }
                }
                match turn {
                    Player::Red => min,
                    Player::Black => max,
                }
            }
        }
    }

    fn evaluate(&self, turn: &Player) -> i32 {
        // By default, only calculate black's value
        let mut sum_red = 0i32;
        let mut sum_black = 0i32;
        for i in 0..MAX_CELLS_SIZE {
            match self.positions[i] {
                None => continue,
                Some(v) => match v {
                    RED_KING => {
                        sum_red += EVALUATE_BASIC[0];
                        sum_red += EVALUATE_KING[i];
                    },
                    RED_ADVISER => {
                        sum_red += EVALUATE_BASIC[1];
                        sum_red += EVALUATE_ADVISER[i];
                    }
                    RED_BISHOP => {
                        sum_red += EVALUATE_BASIC[2];
                        sum_red += EVALUATE_BISHOP[i];
                    },
                    RED_KNIGHT => {
                        sum_red += EVALUATE_BASIC[3];
                        sum_red += EVALUATE_KNIGHT[i];
                    },
                    RED_ROOK => {
                        sum_red += EVALUATE_BASIC[4];
                        sum_red += EVALUATE_ROOK[i];
                    },
                    RED_CANNON => {
                        sum_red += EVALUATE_BASIC[5];
                        sum_red += EVALUATE_CANNON[i];
                    },
                    RED_PAWN => {
                        sum_red += EVALUATE_BASIC[6];
                        sum_red += EVALUATE_PAWN[i];
                    },
                    BLACK_KING => {
                        sum_black += EVALUATE_BASIC[0];
                        sum_black += EVALUATE_KING[89 - i];
                    },
                    BLACK_ADVISER => {
                        sum_black += EVALUATE_BASIC[1];
                        sum_black += EVALUATE_ADVISER[89 - i];
                    },
                    BLACK_BISHOP => {
                        sum_black += EVALUATE_BASIC[2];
                        sum_black += EVALUATE_BISHOP[89 - i];
                    },
                    BLACK_KNIGHT => {
                        sum_black += EVALUATE_BASIC[3];
                        sum_black += EVALUATE_KNIGHT[89 - i];
                    },
                    BLACK_ROOK => {
                        sum_black += EVALUATE_BASIC[4];
                        sum_black += EVALUATE_ROOK[89 - i];
                    },
                    BLACK_CANNON => {
                        sum_black += EVALUATE_BASIC[5];
                        sum_black += EVALUATE_CANNON[89 - i];
                    },
                    BLACK_PAWN => {
                        sum_black += EVALUATE_BASIC[6];
                        sum_black += EVALUATE_PAWN[89 - i];
                    },
                    _ => continue,
                },
            }
        }
        match turn {
            Player::Red => sum_red - sum_black,
            Player::Black => sum_black - sum_red,
        }
    }

    fn temporary_move(&mut self, from: usize, to: usize) -> Option<char> {
        let piece = self.positions[to];
        self.positions[to] = self.positions[from];
        self.positions[from] = None;
        match piece {
            None => (),
            _ => self.pieces_count -= 1,
        }
        piece
    }

    fn recovery(&mut self, from: usize, to: usize, position_to_backup: Option<char>) {
        self.positions[from] = self.positions[to];
        match position_to_backup {
            None => (),
            _ => {
                self.positions[to] = position_to_backup;
                self.pieces_count += 1;
            }
        };
    }

    fn generate_all_moves(&mut self, turn: &Player) -> Vec<MinMaxNode> {
        //        let pieces_size = self.pieces.len();
        //        for i in 0..pieces_size {
        //            //            println!("{:?}", p)
        //            //&self.pieces
        //            self.generate_king(&Player::Red);
        //        }
        // TODO
        Vec::new()
    }

    fn generate_king(&mut self, player: &Player) {
        // 帥 将
        // TODO
        match player {
            Player::Red => {
                println!("red");
            }
            Player::Black => {
                println!("black");
            }
        }
    }

    fn has_piece(&self, p: usize) -> bool {
        self.positions[p] != None
    }

    fn clear(&mut self) {
        // clear pieces
        self.pieces_count = 0usize;
        // clear cells
        for i in 0usize..MAX_CELLS_SIZE {
            self.positions[i] = None;
        }
    }
}

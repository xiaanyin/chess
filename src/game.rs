use game::Player::Black;
use serde;
use serde_derive;
use std::cmp;
use std::collections::{HashMap, HashSet};

/**************************************************************************************************/
/*******************************     BASIC DEFINITION     *****************************************/
/**************************************************************************************************/
// 横向
const WIDTH: usize = 9;
// 纵向
const HEIGHT: usize = 10;
// 最大位置数
const MAX_CELLS_SIZE: usize = WIDTH * HEIGHT;
// 最大棋子数
const MAX_PIECES_SIZE: usize = 32;

const STEP_INCREASE: bool = true;
const STEP_DECREASE: bool = false;
const PROCESS_ROW: bool = true;
const PROCESS_COLUMN: bool = false;

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

/**************************************************************************************************/
/*******************************     INDEX DEFINITION     *****************************************/
/**************************************************************************************************/
const INDEX_ROW: [usize; MAX_CELLS_SIZE] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3,
    3, 3, 3, 3, 4, 4, 4, 4, 4, 4, 4, 4, 4, 5, 5, 5, 5, 5, 5, 5, 5, 5, 6, 6, 6, 6, 6, 6, 6, 6, 6, 7,
    7, 7, 7, 7, 7, 7, 7, 7, 8, 8, 8, 8, 8, 8, 8, 8, 8, 9, 9, 9, 9, 9, 9, 9, 9, 9,
];

const INDEX_COLUMN: [usize; MAX_CELLS_SIZE] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 0, 1, 2, 3, 4, 5, 6, 7, 8, 0, 1, 2, 3, 4, 5, 6, 7, 8, 0, 1, 2, 3, 4,
    5, 6, 7, 8, 0, 1, 2, 3, 4, 5, 6, 7, 8, 0, 1, 2, 3, 4, 5, 6, 7, 8, 0, 1, 2, 3, 4, 5, 6, 7, 8, 0,
    1, 2, 3, 4, 5, 6, 7, 8, 0, 1, 2, 3, 4, 5, 6, 7, 8, 0, 1, 2, 3, 4, 5, 6, 7, 8,
];

const INDEX_ROW_POSITIONS: [[usize; WIDTH]; HEIGHT] = [
    [0, 1, 2, 3, 4, 5, 6, 7, 8],
    [9, 10, 11, 12, 13, 14, 15, 16, 17],
    [18, 19, 20, 21, 22, 23, 24, 25, 26],
    [27, 28, 29, 30, 31, 32, 33, 34, 35],
    [36, 37, 38, 39, 40, 41, 42, 43, 44],
    [45, 46, 47, 48, 49, 50, 51, 52, 53],
    [54, 55, 56, 57, 58, 59, 60, 61, 62],
    [63, 64, 65, 66, 67, 68, 69, 70, 71],
    [72, 73, 74, 75, 76, 77, 78, 79, 80],
    [81, 82, 83, 84, 85, 86, 87, 88, 89],
];

const INDEX_COLUMN_POSITIONS: [[usize; HEIGHT]; WIDTH] = [
    [0, 9, 18, 27, 36, 45, 54, 63, 72, 81],
    [1, 10, 19, 28, 37, 46, 55, 64, 73, 82],
    [2, 11, 20, 29, 38, 47, 56, 65, 74, 83],
    [3, 12, 21, 30, 39, 48, 57, 66, 75, 84],
    [4, 13, 22, 31, 40, 49, 58, 67, 76, 85],
    [5, 14, 23, 32, 41, 50, 59, 68, 77, 86],
    [6, 15, 24, 33, 42, 51, 60, 69, 78, 87],
    [7, 16, 25, 34, 43, 52, 61, 70, 79, 88],
    [8, 17, 26, 35, 44, 53, 62, 71, 80, 89],
];

const RED_KING_POSITIONS: [usize; 9] = [66, 67, 68, 75, 76, 77, 84, 85, 86];

const BLACK_KING_POSITIONS: [usize; 9] = [3, 4, 5, 12, 13, 14, 21, 22, 23];

/**************************************************************************************************/
/*******************************   EVALUATE DEFINITION    *****************************************/
/**************************************************************************************************/
// 帅 士 相 马 车 炮 兵
const EVALUATE_BASIC: [i32; 7] = [0, 40, 40, 88, 200, 96, 9];

const EVALUATE_KING: [i32; MAX_CELLS_SIZE] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, -9, -9, -9, 0, 0, 0, 0, 0, 0, -8, -8, -8, 0, 0, 0, 0, 0, 0, 1, 5, 1, 0, 0, 0,
];

const EVALUATE_ADVISER: [i32; MAX_CELLS_SIZE] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

const EVALUATE_BISHOP: [i32; MAX_CELLS_SIZE] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    -2, 0, 0, 0, 3, 0, 0, 0, -2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

const EVALUATE_KNIGHT: [i32; MAX_CELLS_SIZE] = [
    2, 2, 2, 8, 2, 8, 2, 2, 2, 2, 8, 15, 9, 6, 9, 15, 8, 2, 4, 10, 11, 15, 11, 15, 11, 10, 4, 5,
    20, 12, 19, 12, 19, 12, 20, 5, 2, 12, 11, 15, 16, 15, 11, 12, 2, 2, 10, 13, 14, 15, 14, 13, 10,
    2, 4, 6, 10, 7, 10, 7, 10, 6, 4, 5, 4, 6, 7, 4, 7, 6, 4, 5, -3, 2, 4, 5, -10, 5, 4, 2, -3, 0,
    -3, 2, 0, 2, 0, 2, -3, 0,
];

const EVALUATE_ROOK: [i32; MAX_CELLS_SIZE] = [
    6, 8, 7, 13, 14, 13, 7, 8, 6, 6, 12, 9, 16, 33, 16, 9, 12, 6, 6, 8, 7, 14, 16, 14, 7, 8, 6, 6,
    13, 13, 16, 16, 16, 13, 13, 6, 8, 11, 11, 14, 15, 14, 11, 11, 8, 8, 12, 12, 14, 15, 14, 12, 12,
    8, 4, 9, 4, 12, 14, 12, 4, 9, 4, -2, 8, 4, 12, 12, 12, 4, 8, -2, 5, 8, 6, 12, 0, 12, 6, 8, 5,
    -6, 6, 4, 12, 0, 12, 4, 6, -6,
];

const EVALUATE_CANNON: [i32; MAX_CELLS_SIZE] = [
    4, 4, 0, -5, -6, -5, 0, 4, 4, 2, 2, 0, -4, -7, -4, 0, 2, 2, 1, 1, 0, -5, -4, -5, 0, 1, 1, 0, 3,
    3, 2, 4, 2, 3, 3, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, -1, 0, 3, 0, 4, 0, 3, 0, -1, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 1, 0, 4, 3, 5, 3, 4, 0, 1, 0, 1, 2, 2, 2, 2, 2, 1, 0, 0, 0, 1, 3, 3, 3, 1, 0, 0,
];

const EVALUATE_PAWN: [i32; MAX_CELLS_SIZE] = [
    0, 0, 0, 2, 4, 2, 0, 0, 0, 20, 30, 50, 65, 70, 65, 50, 30, 20, 20, 30, 45, 55, 55, 55, 45, 30,
    20, 20, 27, 30, 40, 42, 40, 30, 27, 20, 10, 18, 22, 35, 40, 35, 22, 18, 10, 3, 0, 4, 0, 7, 0,
    4, 0, 3, -2, 0, -2, 0, 6, 0, -2, 0, -2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0,
];

/**************************************************************************************************/
/*******************************  PIECE MOVES DEFINITION  *****************************************/
/**************************************************************************************************/
lazy_static! {
    static ref MOVES_KING: HashMap<usize, Vec<usize>> = {
        let mut map = HashMap::new();
        map.insert(3, vec![4, 12]);
        map.insert(4, vec![3, 5, 13]);
        map.insert(5, vec![4, 14]);
        map.insert(12, vec![3, 13, 21]);
        map.insert(13, vec![4, 12, 14, 22]);
        map.insert(14, vec![5, 13, 23]);
        map.insert(21, vec![12, 22]);
        map.insert(22, vec![13, 21, 23]);
        map.insert(23, vec![14, 22]);

        map.insert(66, vec![67, 75]);
        map.insert(67, vec![66, 68, 76]);
        map.insert(68, vec![67, 77]);
        map.insert(75, vec![66, 76, 84]);
        map.insert(76, vec![67, 75, 77, 85]);
        map.insert(77, vec![68, 76, 86]);
        map.insert(84, vec![75, 85]);
        map.insert(85, vec![76, 84, 86]);
        map.insert(86, vec![77, 85]);
        map
    };
    static ref MOVES_ADVISER: HashMap<usize, Vec<usize>> = {
        let mut map = HashMap::new();
        map.insert(3, vec![13]);
        map.insert(5, vec![13]);
        map.insert(13, vec![3, 5, 21, 23]);
        map.insert(21, vec![13]);
        map.insert(23, vec![13]);

        map.insert(66, vec![76]);
        map.insert(68, vec![76]);
        map.insert(76, vec![66, 68, 84, 86]);
        map.insert(84, vec![76]);
        map.insert(86, vec![76]);
        map
    };
    static ref MOVES_BISHOP: HashMap<usize, Vec<usize>> = {
        let mut map = HashMap::new();
        map.insert(2, vec![18, 22]);
        map.insert(6, vec![22, 26]);
        map.insert(18, vec![2, 38]);
        map.insert(22, vec![2, 6, 38, 42]);
        map.insert(26, vec![6, 42]);
        map.insert(38, vec![18, 22]);
        map.insert(42, vec![22, 26]);

        map.insert(47, vec![63, 67]);
        map.insert(51, vec![67, 71]);
        map.insert(63, vec![47, 83]);
        map.insert(67, vec![47, 51, 83, 87]);
        map.insert(71, vec![51, 87]);
        map.insert(83, vec![63, 67]);
        map.insert(87, vec![67, 71]);
        map
    };
}

// By default, player is Red, and computer is Black.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
enum Player {
    Red,
    Black,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
struct MinMaxNode {
    piece: char,
    from: usize,
    to: usize,
}

impl MinMaxNode {
    fn new(p: char, f: usize, t: usize) -> MinMaxNode {
        MinMaxNode {
            piece: p,
            from: f,
            to: t,
        }
    }
}

pub struct Board {
    pieces_count: usize,
    positions: [Option<char>; MAX_CELLS_SIZE],
    cache_red_king: usize,
    cache_black_king: usize,
}

impl Board {
    pub fn new() -> Board {
        Board {
            pieces_count: 0usize,
            positions: [None; MAX_CELLS_SIZE],
            cache_red_king: 0usize,
            cache_black_king: 0usize,
        }
    }

    pub fn init_board(&mut self, fen: &str) {
        // rnbakabnr/9/1c5c1/p1p1p1p1p/9/9/P1P1P1P1P/1C5C1/9/RNBAKABNR
        self.clear();
        let mut position = 0usize;
        let mut pieces = 0usize;
        for c in fen.chars() {
            match c {
                '/' => {}
                '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                    position += c.to_digit(10).unwrap() as usize;
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
        let mut all_moves: Vec<MinMaxNode> = self.generate_all_moves();

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
        self.translate(best_move.unwrap().from, best_move.unwrap().to)
    }

    fn translate(&self, from: usize, to: usize) -> String {
        let mut ret: String = String::new();
        ret.push_str(&INDEX_ROW[from].to_string());
        ret.push_str(&INDEX_COLUMN[from].to_string());
        ret.push_str(&INDEX_ROW[to].to_string());
        ret.push_str(&INDEX_COLUMN[to].to_string());
        ret
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
                let mut all_moves: Vec<MinMaxNode> = self.generate_all_moves();
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
                    // TODO
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
                None => {}
                Some(p) => match p {
                    RED_KING => {
                        sum_red += EVALUATE_BASIC[0];
                        sum_red += EVALUATE_KING[i];
                    }
                    RED_ADVISER => {
                        sum_red += EVALUATE_BASIC[1];
                        sum_red += EVALUATE_ADVISER[i];
                    }
                    RED_BISHOP => {
                        sum_red += EVALUATE_BASIC[2];
                        sum_red += EVALUATE_BISHOP[i];
                    }
                    RED_KNIGHT => {
                        sum_red += EVALUATE_BASIC[3];
                        sum_red += EVALUATE_KNIGHT[i];
                    }
                    RED_ROOK => {
                        sum_red += EVALUATE_BASIC[4];
                        sum_red += EVALUATE_ROOK[i];
                    }
                    RED_CANNON => {
                        sum_red += EVALUATE_BASIC[5];
                        sum_red += EVALUATE_CANNON[i];
                    }
                    RED_PAWN => {
                        sum_red += EVALUATE_BASIC[6];
                        sum_red += EVALUATE_PAWN[i];
                    }
                    BLACK_KING => {
                        sum_black += EVALUATE_BASIC[0];
                        sum_black += EVALUATE_KING[89 - i];
                    }
                    BLACK_ADVISER => {
                        sum_black += EVALUATE_BASIC[1];
                        sum_black += EVALUATE_ADVISER[89 - i];
                    }
                    BLACK_BISHOP => {
                        sum_black += EVALUATE_BASIC[2];
                        sum_black += EVALUATE_BISHOP[89 - i];
                    }
                    BLACK_KNIGHT => {
                        sum_black += EVALUATE_BASIC[3];
                        sum_black += EVALUATE_KNIGHT[89 - i];
                    }
                    BLACK_ROOK => {
                        sum_black += EVALUATE_BASIC[4];
                        sum_black += EVALUATE_ROOK[89 - i];
                    }
                    BLACK_CANNON => {
                        sum_black += EVALUATE_BASIC[5];
                        sum_black += EVALUATE_CANNON[89 - i];
                    }
                    BLACK_PAWN => {
                        sum_black += EVALUATE_BASIC[6];
                        sum_black += EVALUATE_PAWN[89 - i];
                    }
                    _ => {}
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
            None => {}
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

    /// 所有棋子可能移动位置生成
    ///
    /// 生成棋盘上所有棋子可能移动的所有位置。
    fn generate_all_moves(&mut self) -> Vec<MinMaxNode> {
        let mut all_moves: Vec<MinMaxNode> = Vec::new();
        for i in 0..MAX_CELLS_SIZE {
            if let Some(p) = self.positions[i] {
                match p {
                    RED_KING => {
                        self.generate_king(&mut all_moves, RED_KING, i);
                    }
                    RED_ADVISER => {
                        self.generate_adviser(&mut all_moves, RED_ADVISER, i);
                    }
                    RED_BISHOP => {
                        self.generate_bishop(&mut all_moves, RED_BISHOP, i);
                    }
                    RED_KNIGHT => {
                        self.generate_knight(&mut all_moves, RED_KNIGHT, i);
                    }
                    RED_ROOK => {
                        self.generate_rook(&mut all_moves, RED_ROOK, i);
                    }
                    RED_CANNON => {
                        self.generate_cannon(&mut all_moves, RED_CANNON, i);
                    }
                    RED_PAWN => {
                        self.generate_pawn(&mut all_moves, &Player::Red, RED_PAWN, i);
                    }
                    BLACK_KING => {
                        self.generate_king(&mut all_moves, BLACK_KING, i);
                    }
                    BLACK_ADVISER => {
                        self.generate_adviser(&mut all_moves, BLACK_ADVISER, i);
                    }
                    BLACK_BISHOP => {
                        self.generate_bishop(&mut all_moves, BLACK_BISHOP, i);
                    }
                    BLACK_KNIGHT => {
                        self.generate_knight(&mut all_moves, BLACK_KNIGHT, i);
                    }
                    BLACK_ROOK => {
                        self.generate_rook(&mut all_moves, BLACK_ROOK, i);
                    }
                    BLACK_CANNON => {
                        self.generate_cannon(&mut all_moves, BLACK_CANNON, i);
                    }
                    BLACK_PAWN => {
                        self.generate_pawn(&mut all_moves, &Player::Black, BLACK_PAWN, i);
                    }
                    _ => {}
                }
            }
        }
        all_moves
    }

    /// 兵（卒）可能移动位置生成
    ///
    /// 生成兵（卒）可能移动的所有位置。
    ///
    /// * `all_moves` - 所有可移动棋子集合。
    /// * `turn` - 红色或者黑色，当前棋子的移动方。
    /// * `piece_from` - 兵（卒）棋子。
    /// * `position_from` - 兵（卒）当前位置。
    fn generate_pawn(
        &mut self,
        all_moves: &mut Vec<MinMaxNode>,
        turn: &Player,
        piece_from: char,
        position_from: usize,
    ) {
        // 0~9
        let row_number: usize = INDEX_ROW[position_from];
        // 0~8
        let column_number: usize = INDEX_COLUMN[position_from];

        let row_positions: &[usize; WIDTH] = &INDEX_ROW_POSITIONS[row_number];
        let column_positions: &[usize; HEIGHT] = &INDEX_COLUMN_POSITIONS[column_number];

        let mut piece_moves: Vec<usize> = Vec::new();

        match turn {
            Player::Red => {
                if row_number > 0usize {
                    piece_moves.push(column_positions[row_number - 1usize]);
                }
                if row_number < 5usize {
                    if column_number > 0usize {
                        piece_moves.push(row_positions[column_number - 1usize]);
                    }
                    if column_number < 8usize {
                        piece_moves.push(row_positions[column_number + 1usize]);
                    }
                }
            }
            Player::Black => {
                if row_number < 9usize {
                    piece_moves.push(column_positions[row_number + 1usize]);
                }
                if row_number > 4usize {
                    if column_number > 0usize {
                        piece_moves.push(row_positions[column_number - 1usize]);
                    }
                    if column_number < 8usize {
                        piece_moves.push(row_positions[column_number + 1usize]);
                    }
                }
            }
        }

        self.generate_general_moves(all_moves, piece_from, position_from, &piece_moves);
    }

    /// 炮可能移动位置生成
    ///
    /// 生成炮可能移动的所有位置。
    ///
    /// * `all_moves` - 所有可移动棋子集合。
    /// * `piece_from` - 炮棋子。
    /// * `position_from` - 炮当前位置。
    fn generate_cannon(
        &mut self,
        all_moves: &mut Vec<MinMaxNode>,
        piece_from: char,
        position_from: usize,
    ) {
        let skip: usize = 1usize;
        let mut piece_moves: Vec<usize> =
            self.generate_piece_move_by_four_direction(position_from, skip);

        self.generate_general_moves(all_moves, piece_from, position_from, &piece_moves);
    }

    /// 车（車）可能移动位置生成
    ///
    /// 生成车（車）可能移动的所有位置。
    ///
    /// * `all_moves` - 所有可移动棋子集合。
    /// * `piece_from` - 车（車）棋子。
    /// * `position_from` - 车（車）当前位置。
    fn generate_rook(
        &mut self,
        all_moves: &mut Vec<MinMaxNode>,
        piece_from: char,
        position_from: usize,
    ) {
        let skip: usize = 0usize;
        let piece_moves: Vec<usize> =
            self.generate_piece_move_by_four_direction(position_from, skip);

        self.generate_general_moves(all_moves, piece_from, position_from, &piece_moves);
    }

    /// 直线移动棋子可能移动位置生成（车/炮）
    ///
    /// 根据对象棋子的上下左右四个方向生成可能移动位置。
    ///
    /// * `start` - 棋子开始移动位置。
    /// * `step` - 坐标增减(STEP_INCREASE或者STEP_DECREASE)。
    /// * `process` - 方向（PROCESS_ROW或者PROCESS_COLUMN）。
    /// * `end` - 棋子最多移动结束位置。
    /// * `piece_moves` - 棋子所有移动坐标。
    /// * `skip` - 跳过棋子数（炮:1,车:0）。
    fn generate_piece_move_by_four_direction(
        &self,
        position_from: usize,
        skip: usize,
    ) -> Vec<usize> {
        // 0~9
        let row_number: usize = INDEX_ROW[position_from];
        // 0~8
        let column_number: usize = INDEX_COLUMN[position_from];

        let row_positions: &[usize; WIDTH] = &INDEX_ROW_POSITIONS[row_number];
        let column_positions: &[usize; HEIGHT] = &INDEX_COLUMN_POSITIONS[column_number];

        let mut piece_moves: Vec<usize> = Vec::new();

        // up
        if row_number > 0usize {
            self.generate_piece_move_by_line(
                row_number - 1usize,
                STEP_DECREASE,
                PROCESS_COLUMN,
                0usize,
                row_positions,
                column_positions,
                &mut piece_moves,
                skip,
            );
        }

        // down
        if row_number < 9usize {
            self.generate_piece_move_by_line(
                row_number + 1usize,
                STEP_INCREASE,
                PROCESS_COLUMN,
                9usize,
                row_positions,
                column_positions,
                &mut piece_moves,
                skip,
            );
        }

        // right
        if column_number < 8usize {
            self.generate_piece_move_by_line(
                column_number + 1usize,
                STEP_INCREASE,
                PROCESS_ROW,
                8usize,
                row_positions,
                column_positions,
                &mut piece_moves,
                skip,
            );
        }

        // left
        if column_number > 0usize {
            self.generate_piece_move_by_line(
                column_number - 1usize,
                STEP_DECREASE,
                PROCESS_ROW,
                0usize,
                row_positions,
                column_positions,
                &mut piece_moves,
                skip,
            );
        }

        piece_moves
    }

    /// 直线移动棋子可能移动位置生成（车/炮）
    ///
    /// 生成车（车/炮）可能移动的所有位置。
    ///
    /// * `start` - 棋子开始移动位置。
    /// * `step` - 坐标增减(STEP_INCREASE或者STEP_DECREASE)。
    /// * `process` - 方向（PROCESS_ROW或者PROCESS_COLUMN）。
    /// * `end` - 棋子最多移动结束位置。
    /// * `row_positions` - 棋子所在行的所有位置坐标。
    /// * `column_positions` - 棋子所在列的所有位置坐标。
    /// * `piece_moves` - 棋子所有移动坐标。
    /// * `skip` - 跳过棋子数（炮:1,车:0）。
    fn generate_piece_move_by_line(
        &self,
        start: usize,
        step: bool,
        process: bool,
        end: usize,
        row_positions: &[usize; WIDTH],
        column_positions: &[usize; HEIGHT],
        piece_moves: &mut Vec<usize>,
        skip: usize,
    ) {
        let mut start_copy: usize = start;
        let mut skip_copy: usize = skip;
        let mut record_mode: bool = true;
        loop {
            let position: usize = match process {
                PROCESS_ROW => row_positions[start_copy],
                PROCESS_COLUMN => column_positions[start_copy],
            };

            if self.positions[position].is_some() {
                if skip_copy == 0usize {
                    piece_moves.push(position);
                    break;
                } else {
                    skip_copy -= 1usize;
                    record_mode = false;
                }
            } else {
                if record_mode {
                    piece_moves.push(position);
                }
            }

            if start_copy == end {
                break;
            } else {
                match step {
                    STEP_INCREASE => start_copy += 1,
                    STEP_DECREASE => start_copy -= 1,
                }
            }
        }
    }

    /// 马（馬）可能移动位置生成
    ///
    /// 生成马（馬）可能移动的所有位置。
    ///
    /// * `all_moves` - 所有可移动棋子集合。
    /// * `piece_from` - 马（馬）棋子。
    /// * `position_from` - 马（馬）当前位置。
    fn generate_knight(
        &mut self,
        all_moves: &mut Vec<MinMaxNode>,
        piece_from: char,
        position_from: usize,
    ) {
        // 0~9
        let row_number = INDEX_ROW[position_from];
        // 0~8
        let column_number = INDEX_COLUMN[position_from];
        // 生成所有位置 (过滤棋盘外位置) (过滤马眼有棋子位置)
        let mut piece_moves: Vec<usize> = Vec::new();

        // up
        if row_number > 1usize && self.is_empty(position_from - WIDTH) {
            if column_number > 0usize {
                piece_moves.push(position_from - 19usize);
            }
            if column_number < 8usize {
                piece_moves.push(position_from - 17usize);
            }
        }

        // down
        if row_number < 8usize && self.is_empty(position_from + WIDTH) {
            if column_number > 0usize {
                piece_moves.push(position_from + 17usize);
            }
            if column_number < 8usize {
                piece_moves.push(position_from + 19usize);
            }
        }

        // right
        if column_number < 7usize && self.is_empty(position_from + 1usize) {
            if row_number > 0usize {
                piece_moves.push(position_from - 7usize);
            }
            if row_number < 9usize {
                piece_moves.push(position_from + 11usize);
            }
        }

        // left
        if column_number > 1usize && self.is_empty(position_from - 1usize) {
            if row_number > 0usize {
                piece_moves.push(position_from - 11usize);
            }
            if row_number < 9usize {
                piece_moves.push(position_from + 7usize);
            }
        }

        self.generate_general_moves(all_moves, piece_from, position_from, &piece_moves);
    }

    /// 相（象）可能移动位置生成
    ///
    /// 生成相（象）可能移动的所有位置。
    ///
    /// * `all_moves` - 所有可移动棋子集合。
    /// * `piece_from` - 相（象）棋子。
    /// * `position_from` - 相（象）当前位置。
    fn generate_bishop(
        &mut self,
        all_moves: &mut Vec<MinMaxNode>,
        piece_from: char,
        position_from: usize,
    ) {
        let piece_moves = MOVES_BISHOP.get(&position_from).unwrap();

        self.generate_general_moves(all_moves, piece_from, position_from, &piece_moves);
    }

    /// 士（仕）可能移动位置生成
    ///
    /// 生成士（仕）可能移动的所有位置。
    ///
    /// * `all_moves` - 所有可移动棋子集合。
    /// * `piece_from` - 士（仕）棋子。
    /// * `position_from` - 士（仕）当前位置。
    fn generate_adviser(
        &mut self,
        all_moves: &mut Vec<MinMaxNode>,
        piece_from: char,
        position_from: usize,
    ) {
        let piece_moves = MOVES_ADVISER.get(&position_from).unwrap();

        self.generate_general_moves(all_moves, piece_from, position_from, &piece_moves);
    }

    /// 帅（将）可能移动位置生成
    ///
    /// 生成帅（将）可能移动的所有位置。
    ///
    /// * `all_moves` - 所有可移动棋子集合。
    /// * `piece_from` - 帅（将）棋子。
    /// * `position_from` - 帅（将）当前位置。
    fn generate_king(
        &mut self,
        all_moves: &mut Vec<MinMaxNode>,
        piece_from: char,
        position_from: usize,
    ) {
        let piece_moves = MOVES_KING.get(&position_from).unwrap();
        self.generate_general_moves(all_moves, piece_from, position_from, &piece_moves);
    }

    /// 固定坐标移动棋子可能移动位置生成
    ///
    /// 生成固定坐标移动棋子可能移动的所有位置。
    ///
    /// * `all_moves` - 所有可移动棋子集合。
    /// * `piece_from` - 棋子。
    /// * `position_from` - 棋子当前位置。
    /// * `piece_moves` - 棋子所有移动位置。
    fn generate_general_moves(
        &mut self,
        all_moves: &mut Vec<MinMaxNode>,
        piece_from: char,
        position_from: usize,
        piece_moves: &Vec<usize>,
    ) {
        for i in 0usize..piece_moves.len() {
            let position_to: usize = piece_moves[i];
            let piece_to: Option<char> = self.positions[position_to];
            if self.is_empty(position_to) || self.is_not_same_side(piece_from, piece_to.unwrap()) {
                let (red_king_position, black_king_position) = match piece_from {
                    RED_KING => (Some(position_to), None),
                    BLACK_KING => (None, Some(position_to)),
                    _ => (None, None),
                };
                if self.king_facing_check(red_king_position, black_king_position) {
                    all_moves.push(MinMaxNode::new(piece_from, position_from, position_to));
                }
            }
        }
    }

    /// 对将检测
    ///
    /// 检测双方是否处于对将状态。
    ///
    /// * `red_king_position` - 红色将军位置，参数可选。若传递None则从棋盘里自动取得，并刷新缓存位置。
    /// * `black_king_position` - 黑色将军位置，参数可选。若传递None则从棋盘里自动取得，并刷新缓存位置。
    fn king_facing_check(
        &mut self,
        red_king_position: Option<usize>,
        black_king_position: Option<usize>,
    ) -> bool {
        let red_king_position: usize = match red_king_position {
            None => self.get_king_position(&Player::Red),
            Some(v) => v,
        };
        let black_king_position: usize = match black_king_position {
            None => self.get_king_position(&Player::Black),
            Some(v) => v,
        };
        let red_king_column: usize = INDEX_COLUMN[red_king_position];
        let black_king_column: usize = INDEX_COLUMN[black_king_position];
        if red_king_column == black_king_column {
            let column_positions = &INDEX_COLUMN_POSITIONS[red_king_column];
            let red_king_row: usize = INDEX_ROW[red_king_position];
            let black_king_row: usize = INDEX_ROW[black_king_position];
            for i in (black_king_row + 1usize)..red_king_row {
                if self.positions[column_positions[i]].is_some() {
                    return true;
                }
            }
            false
        } else {
            true
        }
    }

    /// 将军位置取得
    ///
    /// 取得将军位置，并更新将军位置缓存。
    ///
    /// * `turn` - 红色或者黑色，当前棋子的移动方。
    fn get_king_position(&mut self, turn: &Player) -> usize {
        let cached_king_position: usize = match turn {
            Player::Red => self.cache_red_king,
            Player::Black => self.cache_black_king,
        };
        let target_king: char = match turn {
            Player::Red => RED_KING,
            Player::Black => BLACK_KING,
        };
        let king: Option<char> = self.positions[cached_king_position];
        if king.is_some() && king.unwrap() == target_king {
            cached_king_position
        } else {
            match turn {
                Player::Red => {
                    for position in &RED_KING_POSITIONS {
                        if let Some(piece) = self.positions[*position] {
                            if piece == RED_KING {
                                self.cache_red_king = *position;
                                break;
                            }
                        }
                    }
                    self.cache_red_king
                }
                Player::Black => {
                    for position in &BLACK_KING_POSITIONS {
                        if let Some(piece) = self.positions[*position] {
                            if piece == BLACK_KING {
                                self.cache_black_king = *position;
                                break;
                            }
                        }
                    }
                    self.cache_black_king
                }
            }
        }
    }

    /// 非同色棋子检测
    ///
    /// 检测是否为同一方的棋子。
    ///
    /// * `piece_from` - 检测对象棋子。
    /// * `piece_to` - 检测对象棋子。
    fn is_not_same_side(&self, piece_from: char, piece_to: char) -> bool {
        piece_from.is_ascii_lowercase() != piece_to.is_ascii_lowercase()
    }

    /// 棋子检测
    ///
    /// 检测指定位置是否存在棋子。
    ///
    /// * `position` - 指定位置。
    fn is_empty(&self, position: usize) -> bool {
        self.positions[position].is_none()
    }

    /// 棋盘清理
    ///
    /// 将棋盘上的棋子全部清理。
    fn clear(&mut self) {
        self.pieces_count = 0usize;
        for i in 0usize..MAX_CELLS_SIZE {
            self.positions[i] = None;
        }
    }
}

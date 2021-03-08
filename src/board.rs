use super::gen_tables::*;
use std::mem;
use packed_simd::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Board(pub u64);

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct JsBoard {
    pub upper: i32,
    pub lower: i32
}

pub const SQUARE: u64 = 0x100000001;
const PLAYER: u64 = 0x0000ffff0000ffff;

const START: &str = "qqd1/qdp1/dpp1/4/4/1ppd/1pdq/1dqq";

impl Board {
    pub fn from_desc(desc: &str) -> Self {
        let mut x = 3;
        let mut y = 7;
        let mut out = 0;

        for c in desc.chars() {
            match c {
                'q' => {out |= SQUARE << (x + y * 4)     ; x -= 1},
                'd' => {out |= 1      << (x + y * 4 + 32); x -= 1},
                'p' => {out |= 1      << (x + y * 4)     ; x -= 1},
                '1' => x -= 1, '2' => x -= 2,
                '3' => x -= 3, '4' => x -= 4,
                '/' => {
                    x = 3;
                    y -= 1;
                }
                _ => {}
            }
        }

        Self(out)
    }

    #[cfg(target_arch = "wasm32")]
    pub fn to_js(&self) -> JsBoard {
        unsafe {
            let out = mem::transmute::<_, [i32; 2]>(self.0);

            JsBoard {
                upper: out[0],
                lower: out[1]
            }
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub fn from_js(jsboard: JsBoard) -> Self {
        unsafe {
            Board(mem::transmute::<_, u64>([jsboard.upper, jsboard.lower]))
        }
    }

    pub fn new() -> Self {
        Self::from_desc(START)
    }

    pub fn empty() -> Self {
        Board(0)
    }

    fn occ(&self) -> u32 {
        (self.0 | (self.0 >> 32)) as u32
    }

    fn pawns(&self) -> u32 {
        (self.0 & !(self.0 >> 32)) as u32
    }

    fn drones(&self) -> u32 {
        (!self.0 & (self.0 >> 32)) as u32
    }

    fn queens(&self) -> u32 {
        (self.0 & (self.0 >> 32)) as u32
    }

    pub fn pieces_value(&self) -> i32 {
        let block = (1 << 32) - 1;
        let block2 = block << 32;

        let muls   = u64x2::new(1, 2);

        let mut vec = u64x2::splat(self.0);

        vec &= u64x2::new(block, block2);
        vec = vec.count_ones();
        vec *= muls;

        vec.wrapping_sum() as i32
    }

    pub fn player_value(&self) -> i32 {
        let block = (1 << 16) - 1;
        let mut vec = u64x4::splat(self.0);

        vec &= u64x4::splat(block) << u64x4::new(0, 16, 32, 48);
        vec = vec.count_ones();

        let mut vec: i32x4 = FromCast::from_cast(vec);
        vec *= i32x4::new(-1, 1, -2, 2);

        vec.wrapping_sum()
    }

    fn gen_drone_moves(tables: &Tables, sq: usize, occ: u32) -> u32 {
        let (mask, magic, shift, table) = &tables.drone[sq];
        let mut o = occ;

        o &= *mask;
        o = o.overflowing_mul(*magic).0;
        o >>= *shift;

        table[o as usize]
    }

    fn gen_queen_moves(tables: &Tables, sq: usize, occ: u32) -> u32 {
        let (mask, magic, shift, table) = &tables.queen1[sq];
        let mut o = occ;

        o &= *mask;
        o = o.overflowing_mul(*magic).0;
        o >>= *shift;

        let moves = table[o as usize];

        let (mask, magic, shift, table) = &tables.queen2[sq];
        o = occ;

        o &= *mask;
        o = o.overflowing_mul(*magic).0;
        o >>= *shift;

        moves | table[o as usize]
    }

    fn do_moves(&self, sq: usize, moves: u32, boards: &mut Vec<Board>) {
        let mut piece = self.0;
        piece &= SQUARE << sq;
        piece >>= sq;

        let board = self.0 & !(SQUARE << sq);

        for sq2 in LocStack(moves) {
            let mut b = board;

            b &= !(SQUARE << sq2);
            b |= piece << sq2;

            boards.push(Board(b));
        }
    }

    pub fn gen_moves(&self,
                     player: bool,
                     prev: Board,
                     tables: &Tables,
                     mut out: &mut Vec<Board>)
    {
        out.clear();

        let player = if player {!PLAYER} else {PLAYER};
        let player_board = Board(self.0 & player);

        let player_occ = player_board.occ();
        let occ = self.occ();

        for sq in LocStack(player_board.pawns()) {
            let moves = tables.pawn[sq] & !player_occ;

            self.do_moves(sq, moves, &mut out);
        }

        for sq in LocStack(player_board.drones()) {
            let moves = Self::gen_drone_moves(&tables, sq, occ);
            self.do_moves(sq, moves, &mut out);
        }

        for sq in LocStack(player_board.queens()) {
            let moves = Self::gen_queen_moves(&tables, sq, occ);
            self.do_moves(sq, moves, &mut out);
        }

        let opp = !player;

        out.retain(|b| b.0 & opp == self.0 & opp || b.0 & opp != prev.0 & opp);
    }

    pub fn gen_takes(&self,
                     player: bool,
                     prev: Board,
                     tables: &Tables,
                     mut out: &mut Vec<Board>)
    {
        out.clear();

        let player = if player {!PLAYER} else {PLAYER};
        let player_board = Board(self.0 & player);
        let other_board = Board(self.0 & !player);

        let player_occ = player_board.occ();
        let other_occ = other_board.occ();
        let occ = self.occ();

        for sq in LocStack(player_board.pawns()) {
            let moves = tables.pawn[sq] & !player_occ & other_occ;

            self.do_moves(sq, moves, &mut out);
        }

        for sq in LocStack(player_board.drones()) {
            let moves = Self::gen_drone_moves(&tables, sq, occ) & other_occ;
            self.do_moves(sq, moves, &mut out);
        }

        for sq in LocStack(player_board.queens()) {
            let moves = Self::gen_queen_moves(&tables, sq, occ) & other_occ;
            self.do_moves(sq, moves, &mut out);
        }

        let opp = !player;

        out.retain(|b| b.0 & opp == self.0 & opp || b.0 & opp != prev.0 & opp);
    }

    pub fn get_move(&self, tables: &Tables, mov: &Board) -> (usize, usize) {
        let mut diff = self.0 ^ mov.0;
        diff |= diff >> 32;

        let mut locs = LocStack64(diff);
        let loc1 = locs.next().unwrap();
        let mut loc2 = 32;

        if let Some(l) = locs.next() {
            loc2 = l;
        } else if self.queens() & (1 << loc1) != 0 {
            let moves = Self::gen_queen_moves(&tables, loc1, self.occ());

            if let Some(l2) = LocStack(moves & self.queens()).next() {
                loc2 = l2;
            }
        } else if self.drones() & (1 << loc1) != 0 {
            let moves = Self::gen_drone_moves(&tables, loc1, self.occ());

            if let Some(l2) = LocStack(moves & self.drones()).next() {
                loc2 = l2;
            }
        } else if self.pawns() & (1 << loc1) != 0 {
            let player = if loc1 >= 16 {!PLAYER} else {PLAYER};
            let moves = tables.pawn[loc1] & !(self.occ() & player as u32);

            if let Some(l2) = LocStack(moves & self.pawns()).next() {
                loc2 = l2;
            }
        }

        if self.0 & (SQUARE << loc1) == 0 {
            (loc1, loc2)
        } else {
            (loc2, loc1)
        }
    }

    pub fn do_move(&self, loc1: usize, loc2: usize) -> Board {
        let mut piece = self.0;
        piece &= SQUARE << loc1;
        piece >>= loc1;

        let mut out = self.0;

        out &= !(SQUARE << loc1);
        out &= !(SQUARE << loc2);

        out |= piece << loc2;

        Board(out)
    }

    pub fn do_string_move(&self, s: &str) -> Board {
        let mut chars = s.chars();

        let l1 = chars.next().unwrap();
        let n1 = chars.next().unwrap();
        let l2 = chars.next().unwrap();
        let n2 = chars.next().unwrap();

        let x1 = 3 - (l1 as usize - 97);
        let x2 = 3 - (l2 as usize - 97);
        let y1 = n1 as usize - 49;
        let y2 = n2 as usize - 49;

        self.do_move(x1 + y1 * 4, x2 + y2 * 4)
    }

    pub fn game_end(&self) -> bool {
        self.0 & PLAYER == 0 || self.0 & !PLAYER == 0
    }
}

use std::fmt;

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let player = false;

        let mut y_iter: Box<Iterator<Item = usize>> =
            if player {Box::new(0..8)} else {Box::new((0..8).rev())};

        for y in y_iter {
            write!(f, "{}  ", y + 1);

            let mut x_iter: Box<Iterator<Item = usize>> =
                if player {Box::new(0..4)} else {Box::new((0..4).rev())};

            for x in x_iter {
                let sq = x + y * 4;
                let mut tmp = 0;

                tmp |= (self.0 & (1 << sq)) >> sq;
                tmp |= (self.0 & (1 << (sq + 32))) >> (sq + 31);

                write!(f, "{} ", 
                    match tmp {
                        1 => '^',
                        2 => '*',
                        3 => 'A',
                        _ => '_'
                    }
                )?;
            }
            writeln!(f)?;
        }
        writeln!(f)?;

        if player {
            writeln!(f, "   D C B A");
        } else {
            writeln!(f, "   A B C D");
        }

        Ok(())
    }
}

mod tests {
    extern crate test;
    use test::Bencher;
    use super::*;

    #[bench]
    fn b_pieces_value(b: &mut Bencher) {
        let board = Board::new();

        b.iter(|| test::black_box(&board).pieces_value());
    }

    #[bench]
    fn b_player_value(b: &mut Bencher) {
        let board = Board::new();

        b.iter(|| test::black_box(&board).player_value());
    }

    #[bench]
    fn b_gen_moves(b: &mut Bencher) {
        let mut moves = Vec::new();
        let board = Board(0xea9020804b100000);
        let tables = Tables::new();

        b.iter(|| {test::black_box(&board).gen_moves(false, Board(0), &tables, &mut moves); moves.clone()});
    }

    #[bench]
    fn b_gen_takes(b: &mut Bencher) {
        let mut moves = Vec::new();
        let board = Board(0xea9020804b100000);
        let tables = Tables::new();

        b.iter(|| {test::black_box(&board).gen_takes(false, Board(0), &tables, &mut moves); moves.clone()});
    }
}

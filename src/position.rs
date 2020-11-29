use crate::gen_tables::*;
use crate::board::*;

pub struct Position<'a> {
    pub board: Board,
    prev: Board,
    tables: &'a Tables,
    player: bool,
    score: i32
}

impl<'a> Position<'a> {
    pub fn new(tables: &'a Tables) -> Self {
        Self {
            board: Board::new(),
            prev: Board::empty(),
            tables: tables,
            player: false,
            score: 0
        }
    }

    pub fn to_fen(&self) -> String {
        let mut out = String::new();

        out += &format!("{:016x}", self.board.0);
        out += " ";
        out += &format!("{:016x}", self.prev.0);
        out += " ";

        if self.player {
            out += "2";
        } else {
            out += "1";
        }

        out += " ";
        out += &format!("{}", self.score);

        out
    }

    pub fn from_fen(fen: &str, tables: &'a Tables) -> Self {
        let mut words = fen.split(" ");
        let mut out = Position::new(tables);

        let board = words.next().unwrap();
        let prev  = words.next().unwrap();

        out.board = Board(u64::from_str_radix(board, 16).unwrap());
        out.prev  = Board(u64::from_str_radix(prev , 16).unwrap());

        let player = words.next().unwrap().chars().next().unwrap();

        out.player = player as usize - 49 != 0;
        out.score = words.next().unwrap().parse::<i32>().unwrap();

        out
    }

    pub fn do_move(&mut self, new_board: Board) -> Board {
        let out = self.prev;
        self.prev = self.board;
        self.board = new_board;

        let dscore =
            (self.prev.pieces_value() - self.board.pieces_value()).max(0);

        if self.player {
            self.score -= dscore;
        } else {
            self.score += dscore;
        }

        self.player = !self.player;

        out
    }

    pub fn undo_move(&mut self, prev: Board) -> Board {
        let dscore =
            (self.prev.pieces_value() - self.board.pieces_value()).max(0);

        let out = self.board;
        self.board = self.prev;
        self.prev = prev;

        self.player = !self.player;

        if self.player {
            self.score += dscore;
        } else {
            self.score -= dscore;
        }

        out
    }

    pub fn do_string_move(&mut self, s: &str) -> bool {
        let mut moves = Vec::new();
        self.gen_moves(&mut moves);

        let mov = self.board.do_string_move(s);

        if !moves.contains(&mov) {
            return false;
        }

        self.do_move(mov);
        true
    }

    pub fn get_score(&self) -> i32 {
        if self.player {
            -self.score
        } else {
            self.score
        }
    }

    pub fn get_player(&self) -> bool {
        self.player
    }

    pub fn gen_moves(&self, out: &mut Vec<Board>) {
        self.board.gen_moves(self.player, self.prev, self.tables, out)
    }
}

use std::fmt;

impl fmt::Display for Position<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "p{} {}", self.player as usize + 1, self.get_score());
        writeln!(f);
        writeln!(f, "{}", self.board)?;

        Ok(())
    }
}

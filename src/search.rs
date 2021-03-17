use crate::board::*;
use crate::position::*;

use std::mem;

#[cfg(not(target_arch = "wasm32"))]
use std::time::*;

#[cfg(target_arch = "wasm32")]
use wasm_timer::*;

const TABLE_SIZE: usize = 1_048_573;

pub struct Searcher<'a> {
    pos: Position<'a>,
    moves: Vec<Vec<Board>>,
    transposition: Vec<(Board, bool, usize, i32)>
}

impl<'a> Searcher<'a> {
    pub fn new(pos: Position<'a>) -> Self {
        Self {
            pos,
            moves: Vec::new(),
            transposition: vec![(Board::empty(), false, 0, 0); TABLE_SIZE]
        }
    }

    fn sort_moves(&self, moves: &mut Vec<Board>) {
        moves.sort_by_cached_key(|board| {
            let ind = (board.0 % TABLE_SIZE as u64) as usize;
            let (b, play, _, score) = self.transposition[ind];

            let board_eq = b == *board && self.pos.get_player() == play;

            if board_eq {
                score
            } else {
                0
            }
        });
    }

    fn quiesce(&mut self,
               mut alpha: i32,
               beta: i32) -> i32
    {
        if self.pos.board.game_end() {
            return self.pos.eval() * 100;
        }

        let mut moves = Vec::new();

        self.pos.gen_takes(&mut moves);

        if moves.is_empty() {
            return self.pos.eval();
        }

        for m in moves.iter() {
            let u = self.pos.do_move(*m);
            let score = -self.quiesce(-beta, -alpha);
            self.pos.undo_move(u);

            if score >= beta {
                return beta;
            }
            if score > alpha {
                alpha = score;
            }
        }

        alpha
    }

    fn alphabeta(&mut self,
                 mut alpha: i32,
                 beta: i32,
                 depth: usize) -> i32
    {
        let mut replace = false;

        if self.pos.board.game_end() {
            return self.pos.eval() * 100;
        }

        if depth == 0 {
            return self.quiesce(alpha, beta);
        }

        let ind = (self.pos.board.0 % TABLE_SIZE as u64) as usize;
        let (board, play, depth2, score) = self.transposition[ind];

        let board_eq = self.pos.board == board && self.pos.get_player() == play;

        if depth2 >= depth && board_eq {
            return self.pos.eval() + score;
        } else if depth > depth2 || !board_eq {
            replace = true;
        }

        let mut moves = mem::replace(&mut self.moves[depth], Vec::new());
        let mut retbeta = false;

        self.pos.gen_moves(&mut moves);

        self.sort_moves(&mut moves);

        for m in moves.iter() {
            let u = self.pos.do_move(*m);
            let score = -self.alphabeta(-beta, -alpha, depth - 1);
            self.pos.undo_move(u);

            if score >= beta {
                retbeta = true;
                break;
            }
            if score > alpha {
                alpha = score;
            }
        }

        moves.clear();
        self.moves[depth] = moves;

        let out = if retbeta {beta} else {alpha};

        if replace {
            self.transposition[ind] = (self.pos.board, self.pos.get_player(), depth, out - self.pos.eval())
        }

        out
    }

    fn best_move(&mut self, depth: usize, now: Instant, time: u128)
        -> (Option<Board>, i32)
    {
        if self.pos.board.game_end() {
            return (None, self.pos.eval() * 100);
        }

        let mut ind = 0;
        let mut replace = false;

        if depth > 3 {
            ind = (self.pos.board.0 % TABLE_SIZE as u64) as usize;
            let (board, play, depth2, _) = self.transposition[ind];

            let board_eq = self.pos.board == board && self.pos.get_player() == play;

            if depth > depth2 || !board_eq {
                replace = true;
            }
        }

        let mut best_move = None;
        let mut best_score = -1000000;
        let mut moves = mem::replace(&mut self.moves[depth], Vec::new());

        self.pos.gen_moves(&mut moves);

        if depth > 4 {
            self.sort_moves(&mut moves);
        }

        for m in moves.iter().rev() {
            if now.elapsed().as_millis() >= time {
                return (None, 0);
            }

            let u = self.pos.do_move(*m);
            let score = -self.alphabeta(-1000000, -best_score, depth - 1);

            self.pos.undo_move(u);

            if score > best_score {
                best_move = Some(*m);
                best_score = score;
            }
        }

        if replace {
            self.transposition[ind] = (self.pos.board, self.pos.get_player(), depth, best_score - self.pos.eval())
        }

        moves.clear();
        self.moves[depth] = moves;

        (best_move, best_score)
    }

    pub fn into_position(self) -> Position<'a> {
        self.pos
    }

    pub fn ab_search(&mut self, time: usize) -> (Option<Board>, i32) {
        let time = time as u128;
        let now = Instant::now();

        let mut best = None;
        let mut score = 0;
        let mut d = 1;

        while now.elapsed().as_millis() < time {
            for _ in 0..d + 1 {
                if d >= self.moves.len() {
                    self.moves.push(Vec::new());
                } else {
                    self.moves[d].clear();
                }
            }

            match self.best_move(d, now, time) {
                (None, _) => {
                    return (best, score);
                },
                (m, s) => {
                    best = m;
                    score = s
                }
            }
            d += 1;
        }

        (best, score)
    }
}

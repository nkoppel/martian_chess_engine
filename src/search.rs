use crate::board::*;
use crate::position::*;

use rand::seq::SliceRandom;

use std::mem;
use std::cmp::Ordering;

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

            let board_eq = b == *board && self.pos.get_player() != play;

            if board_eq {
                score
            } else {
                0
            }
        });
    }

    fn quiesce(&mut self, mut alpha: i32, beta: i32) -> i32 {
        if self.pos.board.game_end() {
            return self.pos.eval() * 100;
        }

        let mut moves = Vec::new();

        self.pos.gen_takes(&mut moves);

        if moves.is_empty() {
            return self.pos.eval();
        }

        // a player that gains nothing from taking won't take
        let null_score = self.pos.eval();

        if null_score >= beta {
            return beta + 1;
        }
        if null_score > alpha {
            alpha = null_score;
        }

        for m in moves.into_iter() {
            let u = self.pos.do_move(m);
            let score = -self.quiesce(-beta, -alpha);
            self.pos.undo_move(u);

            if score >= beta {
                return beta + 1;
            }
            if score > alpha {
                alpha = score;
            }
        }

        alpha
    }

    fn alphabeta(&mut self, mut alpha: i32, beta: i32, depth: usize) -> i32 {
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
        }

        let mut moves = mem::take(&mut self.moves[depth]);

        self.pos.gen_moves(&mut moves);
        self.sort_moves(&mut moves);

        for m in moves.iter() {
            let u = self.pos.do_move(*m);
            let score = -self.alphabeta(-beta, -alpha, depth - 1);
            self.pos.undo_move(u);

            if score >= beta {
                moves.clear();
                self.moves[depth] = moves;
                return beta + 1;
            }
            if score > alpha {
                alpha = score;
            }
        }

        moves.clear();
        self.moves[depth] = moves;

        if depth > depth2 || !board_eq {
            self.transposition[ind] = (self.pos.board, self.pos.get_player(), depth, alpha - self.pos.eval())
        }

        alpha
    }

    fn best_moves(&mut self, depth: usize, now: Instant, time: u128)
        -> (Vec<Board>, i32)
    {
        if self.pos.board.game_end() {
            return (Vec::new(), self.pos.eval() * 100);
        }

        let ind = (self.pos.board.0 % TABLE_SIZE as u64) as usize;
        let (board, play, depth2, _) = self.transposition[ind];

        let board_eq = self.pos.board == board && self.pos.get_player() == play;

        let mut best_moves = Vec::new();
        let mut best_score = -1000000;
        let mut moves = mem::take(&mut self.moves[depth]);

        self.pos.gen_moves(&mut moves);
        self.sort_moves(&mut moves);

        for m in moves.iter().rev() {
            if now.elapsed().as_millis() >= time {
                return (Vec::new(), 0);
            }

            let u = self.pos.do_move(*m);
            let score = -self.alphabeta(-1000000, -best_score, depth - 1);

            self.pos.undo_move(u);

            match score.cmp(&best_score) {
                Ordering::Greater => {
                    best_moves.clear();
                    best_moves.push(*m);
                    best_score = score;
                }
                Ordering::Equal => {
                    best_moves.push(*m);
                }
                _ => {},
            }
        }

        if depth > depth2 || !board_eq {
            self.transposition[ind] = (self.pos.board, self.pos.get_player(), depth, best_score - self.pos.eval())
        }

        moves.clear();
        self.moves[depth] = moves;

        (best_moves, best_score)
    }

    pub fn into_position(self) -> Position<'a> {
        self.pos
    }

    pub fn ab_search(&mut self, time: usize) -> (Option<Board>, i32) {
        let time = time as u128;
        let now = Instant::now();

        let mut best = Vec::new();
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

            let (m, s) = self.best_moves(d, now, time);

            if m.is_empty() {
                return (best.choose(&mut rand::thread_rng()).cloned(), score);
            } else {
                best = m;
                score = s
            }
            d += 1;
        }

        if best.is_empty() {
            (None, score)
        } else {
            (best.choose(&mut rand::thread_rng()).cloned(), score)
        }
    }
}

#[allow(unused_imports)]
mod tests {
    extern crate test;
    use test::Bencher;
    use crate::gen_tables::*;
    use super::*;

    #[bench]
    fn b_alphabeta(b: &mut Bencher) {
        let tables = Tables::new();
        let position = Position::new(&tables);

        b.iter(|| {
            let mut searcher = Searcher::new(position.clone());
            searcher.moves = vec![Vec::new(); 7];
            searcher.alphabeta(-1000000, 1000000, 5)
        });
    }

    #[bench]
    fn b_iterative_deepening(b: &mut Bencher) {
        let tables = Tables::new();
        let position = Position::new(&tables);

        b.iter(|| {
            let mut searcher = Searcher::new(position.clone());
            searcher.moves = vec![Vec::new(); 7];

            for d in 0..5 {
                searcher.alphabeta(-1000000, 1000000, d);
            }
            searcher.alphabeta(-1000000, 1000000, 5)
        });
    }
}

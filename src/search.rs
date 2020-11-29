use crate::gen_tables::*;
use crate::board::*;
use crate::position::*;

use std::collections::HashMap;
use std::mem;

use std::io::Write;
use std::time::{Duration, SystemTime};

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
            transposition: vec![(Board::empty(), false, 0, 0); 65537]
        }
    }

    fn alphabeta(&mut self,
                 mut alpha: i32,
                 beta: i32,
                 depth: usize) -> i32
    {
        if self.pos.board.game_end() || depth == 0 {
            return self.pos.get_score();
        }

        let mut moves = mem::replace(&mut self.moves[depth], Vec::new());

        self.pos.gen_moves(&mut moves);

        for m in moves.iter() {
            let u = self.pos.do_move(*m);
            let score = -self.alphabeta(-beta, -alpha, depth - 1);
            self.pos.undo_move(u);

            if score >= beta {
                return beta;
            }
            if score > alpha {
                alpha = score;
            }
        }

        moves.clear();
        self.moves[depth] = moves;

        return alpha;
    }

    fn best_move(&mut self, depth: usize, lastbest: Option<Board>)
        -> (Option<Board>, i32)
    {
        let mut best_move = None;
        let mut best_score = -1000000;
        let mut moves = mem::replace(&mut self.moves[depth], Vec::new());

        self.pos.gen_moves(&mut moves);

        if let Some(lastbest) = lastbest {
            moves.retain(|m| *m != lastbest);
            moves.push(lastbest);
        }

        for m in moves.iter().rev() {
            let u = self.pos.do_move(*m);
            let score = -self.alphabeta(-1000000, -best_score, depth - 1);

            self.pos.undo_move(u);

            if score > best_score {
                best_move = Some(*m);
                best_score = score;
            }
        }

        moves.clear();
        self.moves[depth] = moves;

        return (best_move, best_score);
    }

    pub fn into_position(self) -> Position<'a> {
        self.pos
    }

    pub fn ab_search(&mut self, time: usize) -> (Option<Board>, i32) {
        let time = time as u128;
        let now = SystemTime::now();

        let mut best = None;
        let mut score = 0;
        let mut d = 1;

        while now.elapsed().ok().unwrap().as_millis() < time {
            for i in 0..d + 1 {
                if d >= self.moves.len() {
                    self.moves.push(Vec::new());
                } else {
                    self.moves[d].clear();
                }
            }

            print!("{}", d % 10);
            std::io::stdout().flush();

            match self.best_move(d, best) {
                (None, _) => return (None, score),
                (m, s) => {
                    best = m;
                    score = s
                }
            }
            d += 1;
        }
        println!();
        (best, score)
    }
}

mod gen_tables;
mod board;
mod position;
mod search;

use gen_tables::*;
use board::*;
use position::*;
use search::*;

use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut in_lines = stdin.lock().lines();

    let tables = Tables::new();
    // let mut pos = Position::new(&tables);
    let mut pos = Position::from_fen("ec800137ca600653 0000000000000000 1 0", &tables);

    while !pos.board.game_end() {
        println!("{}", pos.to_fen());
        println!("{}", pos);

        while !pos.do_string_move(&in_lines.next().unwrap().unwrap()) {}

        println!("{}", pos.to_fen());
        println!("{}", pos);

        let mut searcher = Searcher::new(pos);
        let (mov, score) = searcher.ab_search(5000);

        pos = searcher.into_position(); 
        pos.do_move(mov.unwrap());
    }
}

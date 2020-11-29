mod gen_tables;
mod board;
mod position;
mod search;

use gen_tables::*;
use board::*;
use position::*;
use search::*;

fn main() {
    let tables = Tables::new();

    let mut pos = Position::new(&tables);

    while !pos.board.game_end() {
        println!("{}", pos);

        let mut searcher = Searcher::new(pos);
        let (mov, score) = searcher.ab_search(5000);

        pos = searcher.into_position(); 
        pos.do_move(mov.unwrap());
    }
}

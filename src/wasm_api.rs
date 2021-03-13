use crate::gen_tables::*;
use crate::board::*;
use crate::position::*;
use crate::search::*;

use wasm_bindgen::prelude::*;
use lazy_static::*;
use std::sync::Arc;

struct Api {
    position: Position<'static>,
    best_move: Board,
    best_score: i32
}

impl Api {
    fn new() -> Self {
        Self {
            position: Position::new(&TABLES),
            best_move: Board::empty(),
            best_score: 0
        }
    }
}

lazy_static! {
    static ref TABLES: Arc<Tables> = Arc::new(Tables::new());
    static ref API: Arc<Api> = Arc::new(Api::new());
}

#[wasm_bindgen]
pub fn set_position(pos: JsPosition) {
    let mut tmp = API.clone();
    let api = unsafe{Arc::get_mut_unchecked(&mut tmp)};

    api.position = Position::from_js(&TABLES, pos);
}

#[wasm_bindgen]
pub fn get_position() -> JsPosition {
    let mut tmp = API.clone();
    let api = unsafe{Arc::get_mut_unchecked(&mut tmp)};

    api.position.to_js()
}

#[wasm_bindgen]
pub fn get_best_move() -> JsBoard {
    let mut tmp = API.clone();
    let api = unsafe{Arc::get_mut_unchecked(&mut tmp)};

    api.best_move.to_js()
}

#[wasm_bindgen]
pub fn get_best_score() -> i32 {
    let mut tmp = API.clone();
    let api = unsafe{Arc::get_mut_unchecked(&mut tmp)};

    api.best_score
}

#[wasm_bindgen]
pub fn move_is_valid(mov: JsBoard) -> bool {
    let mut tmp = API.clone();
    let api = unsafe{Arc::get_mut_unchecked(&mut tmp)};

    let mut moves = Vec::new();

    api.position.gen_moves(&mut moves);
    moves.contains(&Board::from_js(mov))
}

#[wasm_bindgen]
pub fn do_move(mov: JsBoard) {
    let mut tmp = API.clone();
    let api = unsafe{Arc::get_mut_unchecked(&mut tmp)};

    api.position.do_move(Board::from_js(mov));
}

#[wasm_bindgen]
pub fn do_num_move(sq1: usize, sq2: usize) {
    let mut tmp = API.clone();
    let api = unsafe{Arc::get_mut_unchecked(&mut tmp)};

    api.position.do_num_move(sq1, sq2);
}

#[wasm_bindgen]
pub fn do_string_move(mov: String) -> bool {
    let mut tmp = API.clone();
    let api = unsafe{Arc::get_mut_unchecked(&mut tmp)};

    api.position.do_string_move(&mov)
}

#[wasm_bindgen]
pub fn get_string_move() -> String {
    let mut tmp = API.clone();
    let api = unsafe{Arc::get_mut_unchecked(&mut tmp)};

    stringify_move(api.position.get_move())
}

#[wasm_bindgen]
pub fn get_piece_moves(sq: usize) -> i32 {
    let mut tmp = API.clone();
    let api = unsafe{Arc::get_mut_unchecked(&mut tmp)};

    api.position.gen_piece_moves(sq) as i32
}

#[wasm_bindgen]
pub fn search(time: i32) -> bool {
    let mut tmp = API.clone();
    let api = unsafe{Arc::get_mut_unchecked(&mut tmp)};
    let mut searcher = Searcher::new(api.position.clone());

    let (mov, score) = searcher.ab_search(time as usize);

    if let Some(mov) = mov {
        api.best_move = mov;
        api.best_score = score;
        true
    } else {
        false
    }
}

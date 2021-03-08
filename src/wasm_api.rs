use crate::gen_tables::*;
use crate::board::*;
use crate::position::*;
use crate::search::*;

use wasm_bindgen::prelude::*;
use lazy_static::*;
use std::sync::Arc;

lazy_static! {
    pub static ref TABLES: Arc<Tables> = Arc::new(Tables::new());
}

#[wasm_bindgen]
pub fn move_is_valid(pos: JsPosition, mov: JsBoard) -> bool {
    let mut moves = Vec::new();
    let pos = Position::from_js(&TABLES, pos);

    pos.gen_moves(&mut moves);
    moves.contains(&Board::from_js(mov))
} 

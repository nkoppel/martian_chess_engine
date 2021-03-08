#![feature(test)]

mod gen_tables;
mod board;
mod position;
mod search;

#[cfg(target_arch = "wasm32")]
mod wasm_api;

pub use gen_tables::*;
pub use board::*;
pub use position::*;
pub use search::*;

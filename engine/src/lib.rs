extern crate core;

use crate::common::coord::Coord;
use crate::moves::piece_move::PieceMove;

mod common;
mod fen;
mod board;
mod moves;
mod render;
mod position;

#[no_mangle]
pub extern "C" fn evaluate() -> String {
    return String::from("e2e4");
}

#[no_mangle]
pub extern "C" fn get_available_moves() -> String {
    return String::from("e2,e4");
}
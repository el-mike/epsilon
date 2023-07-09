extern crate core;

mod common;
mod fen;
mod piece;
mod board;
mod render;

use crate::fen::parser;
use crate::board::board::Board;

fn main() {
    let mut board = parser::parse(common::constants::FEN_STARTING_POSITION);

    println!("{}", board);
}

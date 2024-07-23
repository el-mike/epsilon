extern crate core;
mod fen;
mod board;
mod moves;
mod render;
mod position;

use crate::fen::parser;
use crate::board::bitboard::Bitboard;

fn main() {
    let mut board = parser::parse(fen::constants::FEN_STARTING_POSITION);

    let mut bitboard = Bitboard(1);

    println!("{}", bitboard);
    println!("{}", board);
}
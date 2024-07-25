extern crate core;
mod board;
mod fen;
mod moves;
mod position;
mod render;

use crate::board::bitboard::Bitboard;
use crate::fen::parser;

fn main() {
    let mut board = parser::parse(fen::constants::FEN_STARTING_POSITION);

    let mut bitboard = Bitboard(1);

    println!("{}", bitboard);
    println!("{}", board);
}

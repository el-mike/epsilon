extern crate core;

mod common;
mod fen;
mod board;
mod render;

use crate::fen::parser;
use crate::board::board::Board;
use crate::board::piece::{Piece, PieceType};
use crate::board::player::Player;

fn main() {
    let mut board = parser::parse(common::constants::FEN_STARTING_POSITION);

    println!("{}", board);
}

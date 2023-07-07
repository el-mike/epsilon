extern crate core;

use bitflags::Flags;
use crate::fen::parse;
use crate::models::board::Board;
use crate::models::coord::Coord;
use crate::models::piece::{Piece, PieceType};
use crate::models::player::Player;

mod models;
mod fen;

fn main() {
    let starting_position = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";

    let mut board = parse(starting_position);

    println!("{}", board);
}

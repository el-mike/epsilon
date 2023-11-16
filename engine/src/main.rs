extern crate core;

mod common;
mod fen;
mod piece;
mod board;
mod moves;
mod render;
mod position;

use crate::fen::parser;
use crate::common::coord::Coord;
use crate::moves::mover::Mover;
use crate::moves::piece_move::PieceMove;

fn main() {
    let mut board = parser::parse(common::constants::FEN_STARTING_POSITION);

    println!("{}", board);

    let source = Coord::new(4, 1);
    let target = Coord::new(4, 3);

    let mut piece_move = PieceMove::new(source, target, false);

    Mover::make_move(&mut board, &mut piece_move);

    println!("{}", board);
}
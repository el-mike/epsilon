extern crate core;

mod common;
mod fen;
mod piece;
mod board;
mod moves;
mod render;
mod position;

use crate::fen::parser;
use crate::board::board::Board;
use crate::common::coord::Coord;
use crate::moves::piece_move::PieceMove;

fn main() {
    let mut board = parser::parse(common::constants::FEN_STARTING_POSITION);

    println!("{}", board);

    let source = Coord::new(4, 1);
    let target = Coord::new(4, 3);

    let mut piece_move = PieceMove::new(source, target, false);

    board.make_move(&mut piece_move);

    println!("{}", board);
}

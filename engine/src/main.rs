extern crate core;
mod common;
mod fen;
mod board;
mod moves;
mod render;
mod position;

use crate::common::coord::Coord;
use crate::fen::parser;
use crate::moves::mover::Mover;
use crate::moves::piece_move::PieceMove;
use crate::board::bitboard::Bitboard;
use crate::board::piece::{Piece, PieceColor, PieceKind};

fn main() {
    let mut board = parser::parse(common::constants::FEN_STARTING_POSITION);

    let mut bitboard = Bitboard(1);

    println!("{}", bitboard);
    println!("{}", board);
}
extern crate core;
mod common;
mod fen;
mod piece;
mod board;
mod moves;
mod render;
mod position;

use crate::common::coord::Coord;
use crate::fen::parser;
use crate::moves::mover::Mover;
use crate::moves::piece_move::PieceMove;
use crate::board::bitboard::Bitboard;
use crate::piece::piece::Piece;
use crate::piece::piece_color::PieceColor;
use crate::piece::piece_kind::PieceKind;

fn main() {
    let mut board = parser::parse(common::constants::FEN_STARTING_POSITION);

    let mut bitboard = Bitboard(1);

    println!("{}", bitboard);
    println!("{}", board);
}
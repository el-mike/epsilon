extern crate core;
mod bitboard;
mod board;
mod fen;
mod moves;
mod position;
mod render;

use crate::bitboard::bitboard::Bitboard;
use crate::board::piece::PieceColor;
use crate::board::square::{Square, SQUARES};
use crate::fen::parser;
use crate::moves::move_tables;
use crate::render::bitboard::render_moves_for_piece;

fn main() {
    let mut board = parser::parse(fen::constants::FEN_STARTING_POSITION);
    let tables = move_tables::MoveTables::new();

    println!("{}", board);

    render_moves_for_piece(Square::E4, tables.pawn_attacks[PieceColor::White][Square::E4]);
}

extern crate core;
mod bitboard;
mod board;
mod fen;
mod moves;
mod position;
mod render;

use crate::bitboard::bitboard::Bitboard;
use crate::bitboard::direction::KnightDirection;
use crate::bitboard::masks::{NOT_AB_FILE, NOT_GH_FILE};
use crate::board::piece::PieceColor;
use crate::board::square::{Square, SQUARES};
use crate::fen::parser;
use crate::moves::move_tables;
use crate::render::bitboard::render_moves_for_piece;

fn main() {
    let mut board = parser::parse(fen::constants::FEN_STARTING_POSITION);
    let tables = move_tables::MoveTables::new();

    println!("{}", board);

    let square = Square::B4;

    render_moves_for_piece(square, tables.king_attacks[square]);
}

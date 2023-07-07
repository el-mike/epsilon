extern crate core;

use bitflags::Flags;
use crate::models::board::Board;

mod models;
mod fen;

fn main() {
    let starting_position = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";

    let board = Board::new();

    let piece = board.get(1, 1);

    println!("{:?}", piece);

    println!("{}", fen::parse());
}

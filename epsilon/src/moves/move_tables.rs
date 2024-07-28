use crate::bitboard::bitboard::Bitboard;
use std::ops::{Index, IndexMut};
use crate::bitboard::direction::Direction;
use crate::bitboard::masks::{NOT_A_FILE, NOT_H_FILE};
use crate::board::piece::PieceColor;
use crate::board::square::{Square, SQUARES};
use crate::render::bitboard::render_moves_for_piece;

/// One Bitboard of possible moves per every square on the board.
pub type SquareMoveTable = [Bitboard; Bitboard::SIZE as usize];
/// One set of moves per each piece color (player).
pub type MoveTable = [SquareMoveTable; 2];

impl Index<Square> for SquareMoveTable {
    type Output = Bitboard;

    fn index(&self, square: Square) -> &Self::Output {
        return &self[square.as_index()];
    }
}

impl IndexMut<Square> for SquareMoveTable {
    fn index_mut(&mut self, square: Square) -> &mut Self::Output {
        return &mut self[square.as_index()];
    }
}

impl Index<PieceColor> for MoveTable {
    type Output = SquareMoveTable;

    fn index(&self, color: PieceColor) -> &Self::Output {
        return &self[color.as_index()];
    }
}

impl IndexMut<PieceColor> for MoveTable {
    fn index_mut(&mut self, color: PieceColor) -> &mut Self::Output {
        return &mut self[color.as_index()];
    }
}

pub struct MoveTables {
    pub pawn_attacks: MoveTable
}

impl MoveTables {
    pub fn new() -> Self {
        return MoveTables {
               pawn_attacks: MoveTables::calculate_pawn_attacks()
        }
    }

    fn calculate_pawn_attacks() -> MoveTable {
        let mut white_table: SquareMoveTable = [Bitboard(0); Bitboard::SIZE as usize];
        let mut black_table: SquareMoveTable = [Bitboard(0); Bitboard::SIZE as usize];

        Self::iterate_over_board(|square| {
            let position_bitboard = Bitboard::from_bit_index(square.as_bit_index());

            // White pawn.
            // Get NORTH_WEST attack, clear if it lands on H file (overflowing from left side).
            white_table[square] |= (position_bitboard << Direction::NORTH_WEST) & NOT_H_FILE;
            // Get NORTH_EAST attack, clear if it lands on A file (overflowing from right side).
            white_table[square] |= (position_bitboard << Direction::NORTH_EAST) & NOT_A_FILE;

            // Black pawn.
            // Get SOUTH_WEST attack, clear if it lands on H file (overflowing from left side).
            black_table[square] |= (position_bitboard >> Direction::SOUTH_WEST) & NOT_H_FILE;
            // Get SOUTH_EAST attack, clear if it lands on A file (overflowing from right side).
            black_table[square] |= (position_bitboard >> Direction::SOUTH_EAST) & NOT_A_FILE;
        });

        return [white_table, black_table];
    }

    fn iterate_over_board<F: FnMut(Square)>(mut callback: F) {
        for square in SQUARES {
            callback(square);
        }
    }
}
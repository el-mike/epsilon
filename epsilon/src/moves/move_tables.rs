use crate::bitboard::bitboard::Bitboard;
use std::ops::{Index, IndexMut};
use crate::bitboard::direction::{Direction, KnightDirection};
use crate::bitboard::masks::{NOT_A_FILE, NOT_AB_FILE, NOT_H_FILE, NOT_GH_FILE};
use crate::board::piece::PieceColor;
use crate::board::square::{Square, SQUARES};
use crate::render::bitboard::render_moves_for_piece;

/// One Bitboard of possible moves per every square on the board.
pub type MoveTable = [Bitboard; Bitboard::SIZE as usize];
/// Table for pawn moves, which are color-specific (black and white pawns have different moves
/// for the same square.
pub type ColorMoveTable = [MoveTable; 2];

impl Index<Square> for MoveTable {
    type Output = Bitboard;

    fn index(&self, square: Square) -> &Self::Output {
        return &self[square.as_index()];
    }
}

impl IndexMut<Square> for MoveTable {
    fn index_mut(&mut self, square: Square) -> &mut Self::Output {
        return &mut self[square.as_index()];
    }
}

impl Index<PieceColor> for ColorMoveTable {
    type Output = MoveTable;

    fn index(&self, color: PieceColor) -> &Self::Output {
        return &self[color.as_index()];
    }
}

impl IndexMut<PieceColor> for ColorMoveTable {
    fn index_mut(&mut self, color: PieceColor) -> &mut Self::Output {
        return &mut self[color.as_index()];
    }
}


pub struct MoveTables {
    pub pawn_attacks: ColorMoveTable,
    pub knight_attacks: MoveTable,
    pub king_attacks: MoveTable
}

impl MoveTables {
    pub fn new() -> Self {
        return MoveTables {
            pawn_attacks: MoveTables::calculate_pawn_attacks(),
            knight_attacks: MoveTables::calculate_knight_attacks(),
            king_attacks: MoveTables::calculate_king_attacks()
        }
    }

    fn calculate_pawn_attacks() -> ColorMoveTable {
        let mut white_table: MoveTable = [Bitboard(0); Bitboard::SIZE as usize];
        let mut black_table: MoveTable = [Bitboard(0); Bitboard::SIZE as usize];

        Self::iterate_over_squares(|square| {
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

    fn calculate_knight_attacks() -> MoveTable {
        let mut table: MoveTable = [Bitboard(0); Bitboard::SIZE as usize];

        Self::iterate_over_squares(|square| {
            let position_bitboard = Bitboard::from_bit_index(square.as_bit_index());

            table[square] |= (position_bitboard << KnightDirection::TOP_WEST) & NOT_GH_FILE;
            table[square] |= (position_bitboard << KnightDirection::NORTH_WEST) & NOT_H_FILE;
            table[square] |= (position_bitboard << KnightDirection::NORTH_EAST) & NOT_A_FILE;
            table[square] |= (position_bitboard << KnightDirection::TOP_EAST) & NOT_AB_FILE;
            table[square] |= (position_bitboard >> KnightDirection::BOTTOM_EAST) & NOT_AB_FILE;
            table[square] |= (position_bitboard >> KnightDirection::SOUTH_EAST) & NOT_A_FILE;
            table[square] |= (position_bitboard >> KnightDirection::SOUTH_WEST) & NOT_H_FILE;
            table[square] |= (position_bitboard >> KnightDirection::BOTTOM_WEST) & NOT_GH_FILE;
        });

        return table;
    }

    fn calculate_king_attacks() -> MoveTable {
        let mut table: MoveTable = [Bitboard(0); Bitboard::SIZE as usize];

        return table;
    }

    fn iterate_over_squares<F: FnMut(Square)>(mut callback: F) {
        for square in SQUARES {
            callback(square);
        }
    }
}
use std::mem;
use crate::board::bitboard::Bitboard;
use crate::common::coord::Coord;
use crate::piece::piece::Piece;

type ColorBitboards = [Bitboard; 6];

#[derive(Copy, Clone, Debug)]
pub struct BoardState {
    bitboards: [ColorBitboards; 2]
}

impl BoardState {
    /// Returns new BoardState instance, with each Bitboard set to 0 (no pieces).
    pub fn new() -> Self {
        let white_bitboards: ColorBitboards = [0, 0, 0, 0, 0, 0];
        let black_bitboards: ColorBitboards = [0, 0, 0, 0, 0, 0];

        return BoardState {
            bitboards: [white_bitboards, black_bitboards]
        }
    }

    /// Returns Bitboard for given piece.
    pub fn get_bitboard(&self, piece: Piece) -> Bitboard {
        return self.bitboards[piece.color][piece.kind];
    }

    pub fn set_bitboard(&self, coord: &Coord, piece: Piece) {
        let bit_index = coord.x * 8 + coord.y;

        self.bitboards[piece.color][piece.kind] &= 1_u64 << bit_index;
    }


}
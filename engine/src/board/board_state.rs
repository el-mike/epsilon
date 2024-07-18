use std::ops::Index;
use crate::board::bitboard::Bitboard;
use crate::common::coord::Coord;
use crate::piece::piece::{Piece};
use crate::piece::piece_color::PieceColor;
use crate::piece::piece_kind::PieceKind;

type ColorBitboards = [Bitboard; 6];
type Bitboards = [ColorBitboards; 2];

impl Index<PieceKind> for ColorBitboards {
    type Output = Bitboard;

    fn index(&self, kind: PieceKind) -> &Self::Output {
        let index = match kind {
            PieceKind::Pawn => 0,
            PieceKind::Knight => 1,
            PieceKind::Bishop => 2,
            PieceKind::Rook => 3,
            PieceKind::Queen => 4,
            PieceKind::King => 5,
        };

        return &self[index];
    }
}

impl Index<PieceColor> for Bitboards {
    type Output = ColorBitboards;

    fn index(&self, color: PieceColor) -> &Self::Output {
        let index = match color {
            PieceColor::White => 0,
            PieceColor::Black => 1,
        };

        return &self[index];
    }
}

fn get_bit_index_from_coord(coord: &Coord) -> u64 {
    return u64::try_from(coord.x * 8 + coord.y).unwrap();
}

#[derive(Copy, Clone, Debug)]
pub struct BoardState {
    bitboards: Bitboards
}

impl BoardState {
    /// Returns new BoardState instance, with each Bitboard set to 0 (no pieces).
    pub fn new() -> Self {
        let white_bitboards: ColorBitboards = [Bitboard(0), Bitboard(0), Bitboard(0), Bitboard(0), Bitboard(0), Bitboard(0)];
        let black_bitboards: ColorBitboards = [Bitboard(0), Bitboard(0), Bitboard(0), Bitboard(0), Bitboard(0), Bitboard(0)];

        return BoardState {
            bitboards: [white_bitboards, black_bitboards]
        }
    }

    /// Returns Bitboard for given piece.
    pub fn get_bitboard(&self, piece: Piece) -> Bitboard {
        return self.bitboards[piece.color][piece.kind];
    }

    /// Sets Bitboard's bit at given position.
    pub fn set_bitboard(&self, coord: &Coord, piece: Piece) {
        (self.bitboards[piece.color][piece.kind] as Bitboard).set_at(get_bit_index_from_coord(coord));
    }

    pub fn has_at(&self, coord: &Coord, piece: Piece) -> bool {
        return (self.bitboards[piece.color][piece.kind] as Bitboard).is_set_at(get_bit_index_from_coord(coord));
    }
}
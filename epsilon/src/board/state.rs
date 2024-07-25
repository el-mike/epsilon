use crate::bitboard::bitboard::Bitboard;
use crate::board::piece::{PieceColor, PieceKind};
use std::ops::{Index, IndexMut};

pub type ColorBitboards = [Bitboard; 6];
pub type State = [ColorBitboards; 2];

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

impl IndexMut<PieceKind> for ColorBitboards {
    fn index_mut(&mut self, kind: PieceKind) -> &mut Self::Output {
        return match kind {
            PieceKind::Pawn => &mut self[0],
            PieceKind::Knight => &mut self[1],
            PieceKind::Bishop => &mut self[2],
            PieceKind::Rook => &mut self[3],
            PieceKind::Queen => &mut self[4],
            PieceKind::King => &mut self[5],
        };
    }
}

impl Index<PieceColor> for State {
    type Output = ColorBitboards;

    fn index(&self, color: PieceColor) -> &Self::Output {
        let index = match color {
            PieceColor::White => 0,
            PieceColor::Black => 1,
        };

        return &self[index];
    }
}

impl IndexMut<PieceColor> for State {
    fn index_mut(&mut self, kind: PieceColor) -> &mut Self::Output {
        return match kind {
            PieceColor::White => &mut self[0],
            PieceColor::Black => &mut self[1],
        };
    }
}

pub fn get_empty_bitboards() -> State {
    return [
        [
            Bitboard(0),
            Bitboard(0),
            Bitboard(0),
            Bitboard(0),
            Bitboard(0),
            Bitboard(0),
        ],
        [
            Bitboard(0),
            Bitboard(0),
            Bitboard(0),
            Bitboard(0),
            Bitboard(0),
            Bitboard(0),
        ],
    ];
}

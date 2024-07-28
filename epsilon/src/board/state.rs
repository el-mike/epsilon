use crate::bitboard::bitboard::Bitboard;
use crate::board::piece::{PieceColor, PieceKind};
use std::ops::{Index, IndexMut};

pub type ColorBitboards = [Bitboard; 6];
pub type State = [ColorBitboards; 2];

impl Index<PieceKind> for ColorBitboards {
    type Output = Bitboard;

    fn index(&self, kind: PieceKind) -> &Self::Output {
        return &self[kind.as_index()];
    }
}

impl IndexMut<PieceKind> for ColorBitboards {
    fn index_mut(&mut self, kind: PieceKind) -> &mut Self::Output {
        return &mut self[kind.as_index()];
    }
}

impl Index<PieceColor> for State {
    type Output = ColorBitboards;

    fn index(&self, color: PieceColor) -> &Self::Output {
        return &self[color.as_index()];
    }
}

impl IndexMut<PieceColor> for State {
    fn index_mut(&mut self, color: PieceColor) -> &mut Self::Output {
        return &mut self[color.as_index()];
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

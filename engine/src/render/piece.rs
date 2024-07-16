use std::fmt;
use crate::common::fen_symbol::FenSymbol;
use crate::piece::piece::{Piece};
use crate::piece::piece_color::PieceColor;
use crate::piece::piece_kind::PieceKind;

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let base_symbol = match self.kind {
            PieceKind::Pawn => FenSymbol::PAWN,
            PieceKind::Knight => FenSymbol::KNIGHT,
            PieceKind::Bishop => FenSymbol::BISHOP,
            PieceKind::Rook => FenSymbol::ROOK,
            PieceKind::Queen => FenSymbol::QUEEN,
            PieceKind::King => FenSymbol::KING,
        };

        let symbol = if self.color == PieceColor::White {
            base_symbol.to_ascii_uppercase()
        } else {
            base_symbol.to_ascii_lowercase()
        };

        write!(f, "{}", symbol)
    }
}

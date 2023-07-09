use std::fmt;
use crate::common::fen_symbol::FenSymbol;
use crate::board::piece::{Piece, PieceType};
use crate::board::player::Player;

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let base_symbol = match self.piece_type {
            PieceType::None => '_',
            PieceType::Pawn => FenSymbol::PAWN,
            PieceType::Knight => FenSymbol::KNIGHT,
            PieceType::Bishop => FenSymbol::BISHOP,
            PieceType::Rook => FenSymbol::ROOK,
            PieceType::Queen => FenSymbol::QUEEN,
            PieceType::King => FenSymbol::KING,
        };

        let symbol = if self.player == Player::White {
            base_symbol.to_ascii_uppercase()
        } else {
            base_symbol.to_ascii_lowercase()
        };

        write!(f, "{}", symbol)
    }
}

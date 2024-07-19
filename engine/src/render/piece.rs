use crate::common::fen_symbol::FenSymbol;
use crate::piece::piece::{Piece};
use crate::piece::piece_color::PieceColor;
use crate::piece::piece_kind::PieceKind;

pub fn get_symbol_for_piece(piece: &Piece) -> char {
    let base_symbol = match piece.kind {
        PieceKind::Pawn => FenSymbol::PAWN,
        PieceKind::Knight => FenSymbol::KNIGHT,
        PieceKind::Bishop => FenSymbol::BISHOP,
        PieceKind::Rook => FenSymbol::ROOK,
        PieceKind::Queen => FenSymbol::QUEEN,
        PieceKind::King => FenSymbol::KING,
    };

    let symbol = if piece.color == PieceColor::White {
        base_symbol.to_ascii_uppercase()
    } else {
        base_symbol.to_ascii_lowercase()
    };

    return symbol;
}

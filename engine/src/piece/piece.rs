use crate::piece::piece_color::PieceColor;
use crate::piece::piece_kind::PieceKind;

#[derive(Copy, Clone, Debug)]
pub struct Piece {
    pub kind: PieceKind,
    pub color: PieceColor,
}

impl Piece {
    pub fn new(kind: PieceKind, color: PieceColor) -> Self {
        return Self {
            kind,
            color,
        };
    }

    pub fn is_opposite(&self, other_piece: &Piece) -> bool {
        return self.color != other_piece.color;
    }
}

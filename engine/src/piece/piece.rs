use crate::piece::piece_code::{PieceCode, BLACK_COLOR_OFFSET};
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

    pub fn new_empty() -> Self {
        return Self {
            kind: PieceKind::None,
            color: PieceColor::None,
        };
    }

    pub fn is_none(&self) -> bool {
        return self.kind == PieceKind::None;
    }

    pub fn is_opposite(&self, other_piece: &Piece) -> bool {
        return self.color != other_piece.color;
    }

    pub fn get_code(&self) -> PieceCode {
        return self.kind.get_code(self.color);
    }

    pub fn from_code(code: PieceCode) -> Self {
        let color = if code >= BLACK_COLOR_OFFSET {
            PieceColor::Black
        } else {
            PieceColor::White
        };

        let kind = PieceKind::from_code(code);

        return Self {
            color,
            kind,
        };
    }
}

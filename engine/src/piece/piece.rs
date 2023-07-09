use crate::piece::piece_code::{PieceCode, BLACK_COLOR_OFFSET};
use crate::piece::piece_color::PieceColor;
use crate::piece::piece_kind::PieceKind;

pub type PieceValue = u8;

#[derive(Copy, Clone, Debug)]
pub struct Piece {
    pub code: PieceCode,
    pub kind: PieceKind,
    pub color: PieceColor,
}

impl Piece {
    pub fn new(kind: PieceKind, color: PieceColor) -> Self {
        let base_code = kind.get_piece_code();
        let offset = color.get_color_offset();

        let code = base_code + offset;

        return Self { code, kind, color };
    }

    pub fn from_code(code: PieceCode) -> Self {
        let color = if code >= BLACK_COLOR_OFFSET {
            PieceColor::Black
        } else {
            PieceColor::White
        };

        let kind = PieceKind::from_code(code);

        return Self { color, code, kind };
    }

    pub fn value(&self) -> PieceValue {
        return match self.kind {
            PieceKind::None => 0,
            PieceKind::Pawn => 1,
            PieceKind::Knight => 3,
            PieceKind::Bishop => 3,
            PieceKind::Rook => 5,
            PieceKind::Queen => 9,
            PieceKind::King => 20,
        };
    }
}

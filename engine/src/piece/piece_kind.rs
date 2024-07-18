use crate::piece::piece::Piece;
use crate::piece::piece_code;
use crate::piece::piece_code::{PieceCode, BLACK_COLOR_OFFSET};
use crate::piece::piece_color::PieceColor;

pub type PieceValue = u8;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PieceKind {
    Pawn = 0,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl PieceKind {
    pub fn from_code(code: PieceCode) -> Self {
        let code = if code >= BLACK_COLOR_OFFSET {
            code - BLACK_COLOR_OFFSET
        } else {
            code
        };

        match code {
            piece_code::PAWN_CODE => Self::Pawn,
            piece_code::KNIGHT_CODE => Self::Knight,
            piece_code::BISHOP_CODE => Self::Bishop,
            piece_code::ROOK_CODE => Self::Rook,
            piece_code::QUEEN_CODE => Self::Queen,
            piece_code::KING_CODE => Self::King,
            code => panic!("Unknown piece code: {}", code),
        }
    }

    pub fn get_code(&self, color: PieceColor) -> PieceCode {
        let code = match self {
            Self::Pawn => piece_code::PAWN_CODE,
            Self::Knight => piece_code::KNIGHT_CODE,
            Self::Bishop => piece_code::BISHOP_CODE,
            Self::Rook => piece_code::ROOK_CODE,
            Self::Queen => piece_code::QUEEN_CODE,
            Self::King => piece_code::KING_CODE,
        };

        return code + color.get_color_offset();
    }

    pub fn value(&self) -> PieceValue {
        return match self {
            PieceKind::Pawn => 1,
            PieceKind::Knight => 3,
            PieceKind::Bishop => 3,
            PieceKind::Rook => 5,
            PieceKind::Queen => 9,
            PieceKind::King => 20,
        };
    }
}

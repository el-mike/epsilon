use crate::piece::piece_code;
use crate::piece::piece_code::{PieceCode, BLACK_COLOR_OFFSET};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PieceKind {
    None,
    Pawn,
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
            piece_code::NONE_CODE => Self::None,
            piece_code::PAWN_CODE => Self::Pawn,
            piece_code::KNIGHT_CODE => Self::Knight,
            piece_code::BISHOP_CODE => Self::Bishop,
            piece_code::ROOK_CODE => Self::Rook,
            piece_code::QUEEN_CODE => Self::Queen,
            piece_code::KING_CODE => Self::King,
            code => panic!("Unknown piece code: {}", code),
        }
    }

    pub fn get_piece_code(&self) -> PieceCode {
        match self {
            Self::None => piece_code::NONE_CODE,
            Self::Pawn => piece_code::PAWN_CODE,
            Self::Knight => piece_code::KNIGHT_CODE,
            Self::Bishop => piece_code::BISHOP_CODE,
            Self::Rook => piece_code::ROOK_CODE,
            Self::Queen => piece_code::QUEEN_CODE,
            Self::King => piece_code::KING_CODE,
        }
    }
}

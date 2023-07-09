use crate::piece::piece_code::{BLACK_COLOR_OFFSET, PieceCode, WHITE_COLOR_OFFSET};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PieceColor {
    None,
    White,
    Black,
}

impl PieceColor {
    pub fn get_color_offset(&self) -> PieceCode {
        match self {
            PieceColor::White => WHITE_COLOR_OFFSET,
            PieceColor::Black => BLACK_COLOR_OFFSET,
            PieceColor::None => WHITE_COLOR_OFFSET,
        }
    }
}

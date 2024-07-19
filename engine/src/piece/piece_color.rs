#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PieceColor {
    White = 0,
    Black,
}

pub const PIECE_COLORS: [PieceColor; 2] = [PieceColor::White, PieceColor::Black];
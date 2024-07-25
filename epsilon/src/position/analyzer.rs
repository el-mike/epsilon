use crate::board::board::Board;
use crate::board::piece::PieceColor;

pub struct Analyzer<'a> {
    board: &'a Board,
}

impl<'a> Analyzer<'a> {
    pub fn is_king_checked(&self, color: PieceColor) -> bool {
        return false;
    }
}

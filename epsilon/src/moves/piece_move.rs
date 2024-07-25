use crate::board::piece::Piece;
use crate::board::square::Square;

pub struct PieceMove {
    pub source: Square,
    pub target: Square,
    pub takes: bool,
    pub taken_piece: Option<Piece>,
}

impl PieceMove {
    pub fn new(source: Square, target: Square, takes: bool) -> Self {
        return Self {
            source,
            target,
            takes,
            taken_piece: None,
        };
    }
}

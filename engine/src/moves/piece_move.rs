use crate::common::coord::Coord;
use crate::piece::piece::Piece;

pub struct PieceMove {
    pub source: Coord,
    pub target: Coord,
    pub takes: bool,
    pub taken_piece: Option<Piece>,
}

impl PieceMove {
    pub fn new(source: Coord, target: Coord, takes: bool) -> Self {
        return Self {
            source,
            target,
            takes,
            taken_piece: None,
        }
    }
}

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

pub const PIECE_KINDS: [PieceKind; 6] = [PieceKind::Pawn, PieceKind::Knight, PieceKind::Bishop, PieceKind::Rook, PieceKind::Queen, PieceKind::King];
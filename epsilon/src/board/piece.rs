#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PieceColor {
    White = 0,
    Black,
}

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

#[derive(Copy, Clone, Debug)]
pub struct Piece {
    pub kind: PieceKind,
    pub color: PieceColor,
}

impl Piece {
    pub fn new(kind: PieceKind, color: PieceColor) -> Self {
        return Self { kind, color };
    }

    pub fn is_opposite(&self, other_piece: &Piece) -> bool {
        return self.color != other_piece.color;
    }
}

impl PieceColor {
    /// Returns index for given piece color (0 - 1).
    pub fn as_index(self) -> usize {
        return self as usize;
    }
}

impl PieceKind {
    /// Returns index for given piece kind (0 - 5).
    pub fn as_index(self) -> usize {
        return self as usize;
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

pub const PIECE_COLORS: [PieceColor; 2] = [PieceColor::White, PieceColor::Black];
pub const PIECE_KINDS: [PieceKind; 6] = [
    PieceKind::Pawn,
    PieceKind::Knight,
    PieceKind::Bishop,
    PieceKind::Rook,
    PieceKind::Queen,
    PieceKind::King,
];

use crate::models::player;
use crate::models::player::Player;

pub type PieceValue = u8;

#[derive(Copy, Clone, Debug)]
pub enum PieceType {
    None,
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Copy, Clone, Debug)]
pub struct Piece {
    piece_type: PieceType,
    player: Player,
}

impl Piece {
    pub fn new(piece_type: PieceType, player: Player) -> Piece {
        return Self { piece_type, player };
    }

    pub fn value(&self) -> PieceValue {
        return match self.piece_type {
            PieceType::None => 0,
            PieceType::Pawn => 1,
            PieceType::Knight => 3,
            PieceType::Bishop => 3,
            PieceType::Rook => 5,
            PieceType::Queen => 9,
            PieceType::King => 20,
        }
    }
}

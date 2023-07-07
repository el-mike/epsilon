use crate::models::fen::FenPieceSymbol;
use crate::models::player;
use crate::models::player::Player;

pub type PieceValue = u8;

#[derive(Copy, Clone, Debug, PartialEq)]
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
    pub piece_type: PieceType,
    pub player: Player,
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

    pub fn fen_symbol(&self) -> char {
        let symbol = match self.piece_type {
            PieceType::None => FenPieceSymbol::NONE,
            PieceType::Pawn => FenPieceSymbol::PAWN,
            PieceType::Knight => FenPieceSymbol::KNIGHT,
            PieceType::Bishop => FenPieceSymbol::BISHOP,
            PieceType::Rook => FenPieceSymbol::ROOK,
            PieceType::Queen => FenPieceSymbol::QUEEN,
            PieceType::King => FenPieceSymbol::KING,
        };

        return if self.player == Player::White { symbol.to_ascii_uppercase() } else { symbol.to_ascii_lowercase() };
    }
}

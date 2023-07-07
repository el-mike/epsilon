use crate::models::piece::{Piece, PieceType};
use crate::models::player::Player;

const BOARD_SIZE: u8 = 8;

pub struct Board {
    state: [Piece; 64],
}

impl Board {
    pub fn new() -> Self {
        return Board {
            state: [Piece::new(PieceType::Knight, Player::White); 64],
        }
    }

    pub fn get(&self, x: u8, y: u8) -> &Piece {
        let index = usize::from((y * BOARD_SIZE) + x);

        return &self.state[index];
    }
}

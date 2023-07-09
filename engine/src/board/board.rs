use crate::common::coord::Coord;
use crate::board::piece::{Piece, PieceType};
use crate::board::player::Player;

pub enum SquareColor {
    White,
    Black,
}

pub const BOARD_WIDTH: u8 = 8;
const BOARD_SIZE: u8 = 64;

pub struct Board {
    state: [Piece; BOARD_SIZE as usize],
}

impl Board {
    pub fn new() -> Self {
        return Board {
            state: [Piece::new(PieceType::None, Player::None); BOARD_SIZE as usize],
        };
    }

    pub fn get(&self, coord: &Coord) -> &Piece {
        return self
            .state
            .get(self.get_index(coord))
            .expect(format!("Out of bounds coord: x = {}, y = {}", coord.x, coord.y).as_str());
    }

    pub fn set(&mut self, coord: &Coord, piece: Piece) {
        let index = self.get_index(coord);

        self.state[index] = piece;
    }

    pub fn get_square_color(&self, coord: &Coord) -> SquareColor {
        return if coord.y % 2 == 0 && coord.x % 2 == 0 {
            SquareColor::Black
        } else {
            SquareColor::White
        };
    }

    fn get_index(&self, coord: &Coord) -> usize {
        return usize::from((coord.y * BOARD_WIDTH) + coord.x);
    }
}

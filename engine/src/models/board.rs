use crate::models::coord::Coord;
use crate::models::piece::{Piece, PieceType};
use crate::models::player::Player;
use std::fmt;

pub enum SquareColor {
    White,
    Black,
}

const BOARD_WIDTH: u8 = 8;
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
        return if coord.y % 2 == 0 && coord.x % 2 == 0 { SquareColor::Black } else {SquareColor::White};
    }

    fn get_index(&self, coord: &Coord) -> usize {
        return usize::from((coord.y * BOARD_WIDTH) + coord.x);
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::new();

        let mut file = 0;
        let mut rank = 7;

        loop {
            let piece = self.get(&Coord::new(file, rank));

            result.push_str(format!(" {} ", piece.fen_symbol()).as_str());

            file += 1;

            if file == BOARD_WIDTH {
                result.push('\n');

                file = 0;

                if rank != 0 {
                    rank -= 1;
                } else {
                    break;
                }
            }
        }

        write!(f, "{}", result)
    }
}

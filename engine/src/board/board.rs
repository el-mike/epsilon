use crate::common::coord::Coord;
use crate::piece::piece_code::{PieceCode};
use crate::piece::piece_kind::PieceKind;
use crate::piece::piece::Piece;

pub enum SquareColor {
    White,
    Black,
}

pub const BOARD_WIDTH: u8 = 8;
const BOARD_SIZE: u8 = 64;

fn out_of_bounds_index_message(coord: &Coord) -> String {
    return format!("Out of bounds coord: x = {}, y = {}", coord.x, coord.y);
}

pub struct Board {
    state: [PieceCode; BOARD_SIZE as usize],
}

impl Board {
    pub fn new() -> Self {
        return Board {
            state: [0; BOARD_SIZE as usize],
        };
    }

    pub fn get_piece_code(&self, coord: &Coord) -> PieceCode {
        return *self.state.get(self.get_index(coord)).expect(out_of_bounds_index_message(coord).as_str());
    }

    pub fn get_piece(&self, coord: &Coord) -> Piece {
        let code = self.get_piece_code(coord);

        return Piece::from_code(code);
    }

    pub fn set_piece_code(&mut self, coord: &Coord, code: PieceCode) {
        let index = self.get_index(coord);

        self.state[index] = code;
    }

    pub fn set_piece(&mut self, coord: &Coord, piece: &Piece) {
        self.set_piece_code(coord, piece.code);
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

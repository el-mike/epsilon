use crate::board::castling_rights::CastlingRights;
use crate::board::error_messages::out_of_bounds_index_message;
use crate::common::coord::Coord;
use crate::piece::piece::Piece;
use crate::piece::piece_color::PieceColor;
use crate::piece::piece_kind::PieceKind;

pub enum SquareColor {
    White,
    Black,
}

pub const BOARD_WIDTH: usize = 8;
const BOARD_SIZE: usize = 64;

/// Representation of the chess board.
#[derive(Copy, Clone, Debug)]
pub struct Board {
    state: [Piece; BOARD_SIZE],
    pub player_to_move: PieceColor,
    pub white_castling_rights: CastlingRights,
    pub black_castling_rights: CastlingRights,
    pub en_passant_coord: Option<Coord>,
    pub half_move_clock: u8,
    pub full_move_clock: u8
}

impl Board {
    /// Returns new Board instance, filled with "empty" Pieces.
    pub fn new() -> Self {
        return Board {
            state: [Piece::new_empty(); BOARD_SIZE],
            player_to_move: PieceColor::White,
            white_castling_rights: CastlingRights::new(),
            black_castling_rights: CastlingRights::new(),
            en_passant_coord: None,
            half_move_clock: 0,
            full_move_clock: 0
        };
    }

    /// Returns a piece under passed coordinates.
    pub fn get_piece(&self, coord: &Coord) -> &Piece {
        return self
            .state
            .get(self.get_index(coord))
            .expect(out_of_bounds_index_message(coord).as_str());
    }

    /// Sets a piece under passed coordinates.
    pub fn set_piece(&mut self, coord: &Coord, piece: Piece) {
        let index = self.get_index(coord);

        self.state[index] = piece;
    }

    pub fn get_all_by_color(&self, color: PieceColor) -> Vec<Piece> {
        return self
            .state
            .into_iter()
            .filter(|piece| return piece.kind != PieceKind::None && piece.color == color)
            .collect();
    }

    /// Returns true if given coordinates are inside the board, false otherwise.
    pub fn is_inside(&self, coord: &Coord) -> bool {
        return coord.x < BOARD_WIDTH as i8 && coord.y < BOARD_WIDTH as i8;
    }

    /// Returns true if given coordinates contain an actual piece, false otherwise.
    pub fn has_piece(&self, coord: &Coord) -> bool {
        return self.get_piece(coord).kind != PieceKind::None;
    }

    /// Returns the color of a square under given coordinates.
    pub fn get_square_color(&self, coord: &Coord) -> SquareColor {
        return if coord.y % 2 == 0 && coord.x % 2 == 0 {
            SquareColor::Black
        } else {
            SquareColor::White
        };
    }

    /// Returns an index for given coordinates.
    fn get_index(&self, coord: &Coord) -> usize {
        return ((coord.y * BOARD_WIDTH as i8) + coord.x) as usize;
    }
}

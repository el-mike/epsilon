use std::task::Poll::Pending;
use crate::common::coord::Coord;
use crate::moves::piece_move::PieceMove;
use crate::piece::piece::Piece;
use crate::piece::piece_color::PieceColor;
use crate::piece::piece_kind::PieceKind;

pub enum SquareColor {
    White,
    Black,
}

pub const BOARD_WIDTH: usize = 8;
const BOARD_SIZE: usize = 64;

fn out_of_bounds_index_message(coord: &Coord) -> String {
    return format!("Out of bounds coord: x = {}, y = {}", coord.x, coord.y);
}

fn illegal_move_message() -> String {
    return format!("Illegal move detected!");
}

/// Representation of the chess board.
#[derive(Copy, Clone, Debug)]
pub struct Board {
    state: [Piece; BOARD_SIZE],
    player_to_move: PieceColor
}

impl Board {
    /// Returns new Board instance, filled with "empty" Pieces.
    pub fn new() -> Self {
        return Board {
            state: [Piece::new_empty(); BOARD_SIZE],
            player_to_move: PieceColor::White
        };
    }

    /// Allows to set player_to_move.
    pub fn set_player_to_move(&mut self, player_to_move: PieceColor) {
        self.player_to_move = player_to_move;
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

    /// Makes given move and changes the state of the board.
    /// If the moves result in a piece being taken, piece_move struct is updated
    /// with the correct information.
    pub fn make_move(&mut self, piece_move: &mut PieceMove) {
        let source = self.get_piece(&piece_move.source);
        let target = self.get_piece(&piece_move.target);

        if source.color == target.color {
            panic!("{}", illegal_move_message());
        }

        if target.kind != PieceKind::None {
            piece_move.taken_piece = Some(*target);
        }

        self.set_piece(&piece_move.target, *source);
        self.set_piece(&piece_move.source, Piece::new_empty());
    }

    /// Unmakes given move and changes the state of the board.
    /// If the moves contain taken piece, it will be restored to its original
    /// square.
    pub fn unmake_move(&mut self, piece_move: &PieceMove) {
        let target = self.get_piece(&piece_move.target);

        self.set_piece(&piece_move.source, *target);

        if let Some(taken_piece) = piece_move.taken_piece {
            self.set_piece(&piece_move.target, taken_piece);
        } else {
            self.set_piece(&piece_move.target, Piece::new_empty());
        }
    }

    /// Returns an index for given coordinates.
    fn get_index(&self, coord: &Coord) -> usize {
        return ((coord.y * BOARD_WIDTH as i8) + coord.x) as usize;
    }
}

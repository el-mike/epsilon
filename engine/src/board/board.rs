use crate::board::bitboard::Bitboard;
use crate::common::coord::Coord;

use crate::piece::piece::Piece;
use crate::piece::piece_color::{PieceColor, PIECE_COLORS};
use crate::piece::piece_kind::{PieceKind, PIECE_KINDS};

use crate::board::castling_rights::CastlingRights;
use crate::board::bitboards::{Bitboards, get_empty_bitboards};
use crate::board::square_color::SquareColor;

pub const BOARD_WIDTH: usize = 8;
const BOARD_SIZE: usize = 64;


/// @TODO:
/// Simplify when Coord no longer supports negative values.
fn get_bit_index_from_coord(coord: &Coord) -> u8 {
    return u8::try_from(coord.x + coord.y * 8).unwrap();
}

/// Representation of the chess board.
#[derive(Copy, Clone, Debug)]
pub struct Board {
    state: Bitboards,
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
            state: get_empty_bitboards(),
            player_to_move: PieceColor::White,
            white_castling_rights: CastlingRights::new(),
            black_castling_rights: CastlingRights::new(),
            en_passant_coord: None,
            half_move_clock: 0,
            full_move_clock: 0
        };
    }

    /// Returns Bitboard for given piece.
    fn get_bitboard(&self, piece: Piece) -> &Bitboard {
        return &self.state[piece.color][piece.kind];
    }

    /// Sets Bitboard's bit at given position.
    fn set_bitboard(&mut self, coord: &Coord, piece: Piece) {
        self.state[piece.color][piece.kind].set_at(get_bit_index_from_coord(coord));
    }

    /// Returns true if given Piece's Bitboard is set under given coord, false otherwise.
    pub fn has_at(&self, coord: &Coord, piece: Piece) -> bool {
        return self.get_bitboard(piece).is_set_at(get_bit_index_from_coord(coord));
    }

    /// Returns a piece under given coordinates.
    pub fn get_piece(&self, coord: &Coord) -> Option<Piece> {
        let bit_index = get_bit_index_from_coord(coord);

        for color in PIECE_COLORS {
            for kind in PIECE_KINDS {
                let piece = Piece::new(kind, color);

                let bitboard = self.get_bitboard(piece);
                if bitboard.is_set_at(bit_index) {
                    return Some(piece)
                }
            }
        }

        return None
    }

    /// Sets a piece under given coordinates.
    pub fn set_piece(&mut self, coord: &Coord, piece: Piece) {
        self.set_bitboard(coord, piece);
    }

    /// Returns true if given coordinates are inside the board, false otherwise.
    pub fn is_inside(&self, coord: &Coord) -> bool {
        return coord.x < BOARD_WIDTH as i8 && coord.y < BOARD_WIDTH as i8;
    }

    /// Returns the color of a square under given coordinates.
    pub fn get_square_color(&self, coord: &Coord) -> SquareColor {
        return if coord.y % 2 == 0 && coord.x % 2 == 0 {
            SquareColor::Black
        } else {
            SquareColor::White
        };
    }
}

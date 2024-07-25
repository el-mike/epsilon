use crate::board::bitboard::Bitboard;
use crate::board::castling_rights::CastlingRights;
use crate::board::piece::{Piece, PieceColor, PIECE_COLORS, PIECE_KINDS};
use crate::board::square::{Square, SquareColor};
use crate::board::state::{get_empty_bitboards, State};

pub const BOARD_WIDTH: u8 = 8;
const BOARD_SIZE: u8 = 64;

/// Representation of the chess board.
#[derive(Clone, Debug)]
pub struct Board {
    state: State,
    pub player_to_move: PieceColor,
    pub white_castling_rights: CastlingRights,
    pub black_castling_rights: CastlingRights,
    pub en_passant_coord: Option<Square>,
    pub half_move_clock: u8,
    pub full_move_clock: u8,
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
            full_move_clock: 0,
        };
    }

    /// Returns a reference to Bitboard for given piece.
    fn get_bitboard_for_piece(&self, piece: Piece) -> Bitboard {
        return self.state[piece.color][piece.kind];
    }

    /// Returns a piece under given coordinates.
    pub fn get_piece(&self, square: Square) -> Option<Piece> {
        let bit_index = square.as_bit_index();

        for color in PIECE_COLORS {
            for kind in PIECE_KINDS {
                let piece = Piece::new(kind, color);

                let bitboard = self.get_bitboard_for_piece(piece);
                if bitboard.is_set_at(bit_index) {
                    return Some(piece);
                }
            }
        }

        return None;
    }

    /// Returns true if given Piece's exists under given coord, false otherwise.
    pub fn has_at(&self, square: Square, piece: Piece) -> bool {
        let bitboard = self.get_bitboard_for_piece(piece);
        return bitboard.is_set_at(square.as_bit_index());
    }

    /// Sets a piece under given square.
    pub fn set_piece(&mut self, square: Square, piece: Piece) {
        self.state[piece.color][piece.kind] =
            self.state[piece.color][piece.kind].set_at(square.as_bit_index());
    }

    /// Unsets a piece under given square.
    pub fn unset_piece(&mut self, square: Square) {
        if let Some(piece) = self.get_piece(square) {
            self.state[piece.color][piece.kind] =
                self.state[piece.color][piece.kind].unset_at(square.as_bit_index());
        }
    }

    /// Returns the color of a square under given coordinates.
    pub fn get_square_color(&self, square: Square) -> SquareColor {
        return if square.as_bit_index() % 2 == 0 {
            SquareColor::Black
        } else {
            SquareColor::White
        };
    }
}

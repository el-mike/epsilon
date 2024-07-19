use crate::board::board::Board;
use crate::moves::error_messages::illegal_move_message;
use crate::moves::piece_move::PieceMove;
use crate::piece::piece::Piece;
use crate::piece::piece_kind::PieceKind;

pub struct Mover {}

impl Mover {
    /// Makes given move and changes the state of the board.
    /// If the moves result in a piece being taken, piece_move struct is updated
    /// with the correct information.
    pub fn make_move(board: &mut Board, piece_move: &mut PieceMove) {
        // @TODO:
        // Add safety.
        let source = board.get_piece(&piece_move.source).unwrap();
        let target = board.get_piece(&piece_move.target).unwrap();

        if source.color == target.color {
            panic!("{}", illegal_move_message());
        }

        // if target.kind != PieceKind::None {
        //     piece_move.taken_piece = Some(*target);
        // }

        board.set_piece(&piece_move.target, source);
    }

    /// Unmakes given move and changes the state of the board.
    /// If the moves contain taken piece, it will be restored to its original
    /// square.
    pub fn unmake_move(board: &mut Board, piece_move: &PieceMove) {
        // @TODO:
        // Add safety.
        let target = board.get_piece(&piece_move.target).unwrap();

        board.set_piece(&piece_move.source, target);

        if let Some(taken_piece) = piece_move.taken_piece {
            board.set_piece(&piece_move.target, taken_piece);
        }
    }
}
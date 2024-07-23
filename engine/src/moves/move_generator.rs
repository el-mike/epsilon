use crate::board::board::Board;
use crate::board::piece::{Piece, PieceKind};
use crate::board::square::{Square};
use crate::moves::piece_move::PieceMove;

pub struct MoveGenerator {}

impl MoveGenerator {
    pub fn generate_moves(&self, board: &Board, square: Square) -> Vec<PieceMove> {
        let source_piece = match board.get_piece(square) {
            Some(p) => p,
            None => return vec![],
        };

        let moves: Vec<PieceMove> = match source_piece.kind {
            PieceKind::Pawn => self.generate_pawn_moves(board, square, &source_piece),
            PieceKind::Knight => self.generate_knight_moves(square, &source_piece),
            PieceKind::Bishop => self.generate_bishop_moves(square, &source_piece),
            PieceKind::Rook => self.generate_rook_moves(square, &source_piece),
            PieceKind::Queen => self.generate_queen_moves(square, &source_piece),
            PieceKind::King => self.generate_king_moves(square, &source_piece),
        };

        return moves;
    }

    fn generate_pawn_moves(&self, board: &Board, square: Square, piece: &Piece) -> Vec<PieceMove> {
        let pseudo_legal_moves = self.generate_sliding_moves(board, square);

        return pseudo_legal_moves;
    }

    fn generate_knight_moves(&self, square: Square, piece: &Piece) -> Vec<PieceMove> {
        return Vec::new();
    }

    fn generate_bishop_moves(&self, square: Square, piece: &Piece) -> Vec<PieceMove> {
        return Vec::new();
    }

    fn generate_rook_moves(&self, square: Square, piece: &Piece) -> Vec<PieceMove> {
        return Vec::new();
    }

    fn generate_queen_moves(&self, square: Square, piece: &Piece) -> Vec<PieceMove> {
        return Vec::new();
    }

    fn generate_king_moves(&self, square: Square, piece: &Piece) -> Vec<PieceMove> {
        return Vec::new();
    }

    fn generate_sliding_moves(&self, board: &Board, square: Square) -> Vec<PieceMove> {
        let mut moves: Vec<PieceMove> = Vec::new();

        return moves;
    }
}

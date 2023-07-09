use crate::board::board::Board;
use crate::common::coord::Coord;
use crate::piece::piece::Piece;
use crate::moves::piece_move::PieceMove;
use crate::piece::piece_kind::PieceKind;

pub struct MoveGenerator<'a> {
    pub board: &'a Board,
}

impl<'a> MoveGenerator<'a> {
    pub fn generate_moves(&self, coord: &Coord) -> Vec<PieceMove> {
        let source_piece = self.board.get_piece(coord);

        let moves: Vec<PieceMove> = match source_piece.kind {
            PieceKind::None => Vec::new(),
            PieceKind::Pawn => self.generate_pawn_moves(coord, source_piece),
            PieceKind::Knight => self.generate_knight_moves(coord, source_piece),
            PieceKind::Bishop => self.generate_bishop_moves(coord, source_piece),
            PieceKind::Rook => self.generate_rook_moves(coord, source_piece),
            PieceKind::Queen => self.generate_queen_moves(coord, source_piece),
            PieceKind::King => self.generate_king_moves(coord, source_piece),
        };

        return moves;
    }

    fn generate_pawn_moves(&self, coord: &Coord, piece: &Piece) -> Vec<PieceMove> {
        let pseudo_legal_moves = self.generate_sliding_moves(coord, &Coord::NORTH);

        return pseudo_legal_moves;
    }

    fn generate_knight_moves(&self, coord: &Coord, piece: &Piece) -> Vec<PieceMove> {
        return Vec::new();
    }

    fn generate_bishop_moves(&self, coord: &Coord, piece: &Piece) -> Vec<PieceMove> {
        return Vec::new();
    }

    fn generate_rook_moves(&self, coord: &Coord, piece: &Piece) -> Vec<PieceMove> {
        return Vec::new();
    }

    fn generate_queen_moves(&self, coord: &Coord, piece: &Piece) -> Vec<PieceMove> {
        return Vec::new();
    }

    fn generate_king_moves(&self, coord: &Coord, piece: &Piece) -> Vec<PieceMove> {
        return Vec::new();
    }

    fn generate_sliding_moves(&self, coord: &Coord, direction: &Coord) -> Vec<PieceMove> {
        let source_piece = self.board.get_piece(coord);

        let mut moves: Vec<PieceMove> = Vec::new();

        let mut i = 1;

        loop {
            let next_coord = coord.apply_direction(direction, i);

            let target_piece = self.board.get_piece(&next_coord);

            if target_piece.color == source_piece.color {
                break;
            }

            let takes = !target_piece.is_none();

            let piece_move = PieceMove::new(coord.clone(), next_coord, takes);
            moves.push(piece_move);

            i += 1;
        }

        return moves;
    }
}

use crate::common::coord::Coord;
use crate::common::fen_symbol::FenSymbol;
use crate::board::board::Board;
use crate::piece::piece::Piece;
use crate::piece::piece_kind::{PieceKind};
use crate::piece::piece_code::PieceCode;
use crate::piece::piece_color::PieceColor;

const FEN_RANK_BREAK: char = '/';

pub fn parse(fen: &str) -> Board {
    let mut board = Board::new();

    let mut file: u8 = 0;

    // FEN notation starts with the top rank from the White's perspective, which is the
    // seventh rank.
    let mut rank: u8 = 7;

    for c in fen.chars() {
        if c == FEN_RANK_BREAK {
            file = 0;
            rank -= 1;

            continue;
        }

        if c.is_digit(10) {
            let distance = c.to_digit(10).expect("Unknown FEN symbol!");

            file += distance as u8;

            continue;
        }

        let piece = from_fen_symbol(c);

        board.set_piece(&Coord::new(file, rank), &piece);

        file += 1;
    }

    return board;
}

fn from_fen_symbol(symbol: char) -> Piece {
    let kind = match symbol.to_ascii_lowercase() {
        FenSymbol::PAWN => PieceKind::Pawn,
        FenSymbol::KNIGHT => PieceKind::Knight,
        FenSymbol::BISHOP => PieceKind::Bishop,
        FenSymbol::ROOK => PieceKind::Rook,
        FenSymbol::QUEEN => PieceKind::Queen,
        FenSymbol::KING => PieceKind::King,
        _ => PieceKind::None,
    };

    let color = if symbol.is_uppercase() {
        PieceColor::White
    } else {
        PieceColor::Black
    };

    return Piece::new(kind, color);
}

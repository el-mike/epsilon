use crate::common::coord::Coord;
use crate::common::fen_symbol::FenSymbol;
use crate::board::board::Board;
use crate::board::piece::{Piece, PieceType};
use crate::board::player::Player;

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

        board.set(&Coord::new(file, rank), piece);

        file += 1;
    }

    return board;
}

fn from_fen_symbol(symbol: char) -> Piece {
    let piece_type = match symbol.to_ascii_lowercase() {
        FenSymbol::PAWN => PieceType::Pawn,
        FenSymbol::KNIGHT => PieceType::Knight,
        FenSymbol::BISHOP => PieceType::Bishop,
        FenSymbol::ROOK => PieceType::Rook,
        FenSymbol::QUEEN => PieceType::Queen,
        FenSymbol::KING => PieceType::King,
        _ => PieceType::None,
    };

    let player = if symbol.is_uppercase() {
        Player::White
    } else {
        Player::Black
    };

    return Piece { piece_type, player };
}

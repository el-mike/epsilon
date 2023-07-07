use crate::models::board::Board;
use crate::models::coord::Coord;
use crate::models::fen::FenPieceSymbol;
use crate::models::piece::{Piece, PieceType};
use crate::models::player::Player;

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
        FenPieceSymbol::PAWN => PieceType::Pawn,
        FenPieceSymbol::KNIGHT => PieceType::Knight,
        FenPieceSymbol::BISHOP => PieceType::Bishop,
        FenPieceSymbol::ROOK => PieceType::Rook,
        FenPieceSymbol::QUEEN => PieceType::Queen,
        FenPieceSymbol::KING => PieceType::King,
        _ => PieceType::None,
    };

    let player = if symbol.is_uppercase() { Player::White } else { Player::Black };

    return Piece {
        piece_type,
        player
    };
}

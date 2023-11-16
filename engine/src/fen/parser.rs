use crate::common::coord::Coord;
use crate::common::fen_symbol::FenSymbol;
use crate::board::board::Board;
use crate::board::castling_rights::CastlingRights;
use crate::piece::piece::Piece;
use crate::piece::piece_kind::{PieceKind};
use crate::piece::piece_color::PieceColor;

const FEN_RANK_BREAK: char = '/';

pub fn parse(fen: &str) -> Board {
    let mut board = Board::new();

    let segments: Vec<&str> = fen.split_whitespace().collect();

    parse_piece_placement(segments[0], &mut board);

    if segments.get(1).is_some() {
        parse_player_to_move(segments[1], &mut board);
    }

    if segments.get(2).is_some() {
        parse_castling_rights(segments[2], &mut board);
    }

    return board;
}

fn parse_piece_placement(segment: &str, board: &mut Board) {
    let mut file: u8 = 0;

    // FEN notation starts with the top rank from the White's perspective, which is the
    // seventh rank.
    let mut rank: u8 = 7;

    for c in segment.chars() {
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

        let piece = get_piece_from_fen_symbol(c);

        board.set_piece(&Coord::new(file as i8, rank as i8), piece);

        file += 1;
    }
}

fn parse_player_to_move(segment: &str, board: &mut Board) {
    let chars: Vec<char> = segment.chars().collect();

    if chars[0] == FenSymbol::BLACK_TO_MOVE {
        board.set_player_to_move(PieceColor::Black);
    } else {
        board.set_player_to_move(PieceColor::White);
    }
}

fn parse_castling_rights(segment: &str, board: &mut Board) {
    let chars: Vec<char> = segment.chars().collect();

    if chars.len() == 1 && chars[0] == FenSymbol::EMPTY_SEGMENT {
        return;
    }

    let mut white_castling_rights = CastlingRights::new();
    let mut black_castling_rights = CastlingRights::new();

    white_castling_rights.king_side = chars.contains(&FenSymbol::KING_SIDE_CASTLE_WHITE);
    white_castling_rights.queen_side = chars.contains(&FenSymbol::QUEEN_SIDE_CASTLE_WHITE);

    black_castling_rights.king_side = chars.contains(&FenSymbol::KING_SIDE_CASTLE_BLACK);
    black_castling_rights.queen_side = chars.contains(&FenSymbol::QUEEN_SIDE_CASTLE_BLACK);

    board.set_castling_rights(PieceColor::White, white_castling_rights);
    board.set_castling_rights(PieceColor::Black, black_castling_rights);
}

fn get_piece_from_fen_symbol(symbol: char) -> Piece {
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

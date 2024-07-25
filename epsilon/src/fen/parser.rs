use crate::board::algebraic_coord::AlgebraicCoord;
use crate::board::board::Board;
use crate::board::castling_rights::CastlingRights;
use crate::board::piece::{Piece, PieceColor, PieceKind};
use crate::board::square::Square;
use crate::fen::fen_symbol::FenSymbol;

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

    if segments.get(3).is_some() {
        parse_en_passant(segments[3], &mut board);
    }

    if segments.get(4).is_some() {
        parse_half_move_clock(segments[4], &mut board);
    }

    if segments.get(5).is_some() {
        parse_full_move_clock(segments[5], &mut board);
    }

    return board;
}

fn parse_piece_placement(segment: &str, board: &mut Board) {
    let mut file: u8 = 0;

    // FEN notation starts with the top rank from the White's perspective, which is the
    // eight rank (seventh index).
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

        board.set_piece(Square::from_board_coords(file, rank), piece);

        file += 1;
    }
}

fn parse_player_to_move(segment: &str, board: &mut Board) {
    let chars: Vec<char> = segment.chars().collect();

    if chars[0] == FenSymbol::BLACK_TO_MOVE {
        board.player_to_move = PieceColor::Black;
    } else {
        board.player_to_move = PieceColor::White;
    }
}

fn parse_castling_rights(segment: &str, board: &mut Board) {
    let chars: Vec<char> = segment.chars().collect();

    if chars[0] == FenSymbol::EMPTY_SEGMENT {
        return;
    }

    let mut white_castling_rights = CastlingRights::new();
    let mut black_castling_rights = CastlingRights::new();

    white_castling_rights.king_side = chars.contains(&FenSymbol::KING_SIDE_CASTLE_WHITE);
    white_castling_rights.queen_side = chars.contains(&FenSymbol::QUEEN_SIDE_CASTLE_WHITE);

    black_castling_rights.king_side = chars.contains(&FenSymbol::KING_SIDE_CASTLE_BLACK);
    black_castling_rights.queen_side = chars.contains(&FenSymbol::QUEEN_SIDE_CASTLE_BLACK);

    board.white_castling_rights = white_castling_rights;
    board.black_castling_rights = black_castling_rights;
}

fn parse_en_passant(segment: &str, board: &mut Board) {
    if segment.chars().nth(0).expect("Incorrect algebraic coord") == FenSymbol::EMPTY_SEGMENT {
        return;
    }

    let coord = AlgebraicCoord::from_string(segment);

    board.en_passant_coord = Some(coord.to_square())
}

fn parse_half_move_clock(segment: &str, board: &mut Board) {
    if segment.chars().nth(0).expect("Incorrect half move clock") == FenSymbol::EMPTY_SEGMENT {
        return;
    }

    if let Ok(clock) = segment.parse::<u8>() {
        board.half_move_clock = clock
    }
}

fn parse_full_move_clock(segment: &str, board: &mut Board) {
    if segment.chars().nth(0).expect("Incorrect full move clock") == FenSymbol::EMPTY_SEGMENT {
        return;
    }

    if let Ok(clock) = segment.parse::<u8>() {
        board.full_move_clock = clock
    }
}

fn get_piece_from_fen_symbol(symbol: char) -> Piece {
    let kind = match symbol.to_ascii_lowercase() {
        FenSymbol::PAWN => PieceKind::Pawn,
        FenSymbol::KNIGHT => PieceKind::Knight,
        FenSymbol::BISHOP => PieceKind::Bishop,
        FenSymbol::ROOK => PieceKind::Rook,
        FenSymbol::QUEEN => PieceKind::Queen,
        FenSymbol::KING => PieceKind::King,
        _ => panic!("Incorrect FEN symbol: {symbol}"),
    };

    let color = if symbol.is_uppercase() {
        PieceColor::White
    } else {
        PieceColor::Black
    };

    return Piece::new(kind, color);
}

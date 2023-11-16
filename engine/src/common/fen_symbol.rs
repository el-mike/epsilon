pub struct FenSymbol;

impl FenSymbol {
    pub const EMPTY_SEGMENT: char = '-';
    pub const PAWN: char = 'p';
    pub const KNIGHT: char = 'n';
    pub const BISHOP: char = 'b';
    pub const ROOK: char = 'r';
    pub const QUEEN: char = 'q';
    pub const KING: char = 'k';
    pub const WHITE_TO_MOVE: char = 'w';
    pub const BLACK_TO_MOVE: char = 'b';
    pub const KING_SIDE_CASTLE_WHITE: char = 'K';
    pub const QUEEN_SIDE_CASTLE_WHITE: char = 'Q';
    pub const KING_SIDE_CASTLE_BLACK: char = 'k';
    pub const QUEEN_SIDE_CASTLE_BLACK: char = 'q';
}

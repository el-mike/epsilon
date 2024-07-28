use crate::bitboard::bitboard::Bitboard;

/// All the following masks apply to Little Endian File-Rank mapping.
/// They are defined as hexadecimal numbers (every digit cover 4 bits).
pub const A_FILE: Bitboard = Bitboard(0x0101010101010101);
pub const H_FILE: Bitboard = Bitboard(0x8080808080808080);
pub const FIRST_RANK: Bitboard = Bitboard(0x00000000000000FF);
pub const LAST_RANK: Bitboard = Bitboard(0xFF00000000000000);
pub const NOT_A_FILE: Bitboard = Bitboard(0xFEFEFEFEFEFEFEFE);
pub const NOT_H_FILE: Bitboard = Bitboard(0x7F7F7F7F7F7F7F7F);
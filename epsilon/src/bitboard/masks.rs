use crate::bitboard::bitboard::Bitboard;

/// All the following masks apply to Little Endian Rank-File mapping (enumerated rank by rank).
/// They are defined as hexadecimal numbers (every digit cover 4 nibble, which is 4 bits).
/// Note, that hexadecimals in Rust are Big Endian, therefore the order of the bytes
/// is reversed in related to the LERF mapping.
/// For example, considering first rank with following bits:
/// 0 0 1 1  1 1 1 1
/// a b c d  e f g h (files)
/// 1 2 3 4  5 6 7 8 (bits)
/// We get 0xFC (second nibble first, as per Big Endian notation).
pub const A_FILE: Bitboard = Bitboard(0x0101010101010101);
pub const H_FILE: Bitboard = Bitboard(0x8080808080808080);
pub const FIRST_RANK: Bitboard = Bitboard(0x00000000000000FF);
pub const LAST_RANK: Bitboard = Bitboard(0xFF00000000000000);
pub const NOT_A_FILE: Bitboard = Bitboard(0xFEFEFEFEFEFEFEFE);
pub const NOT_H_FILE: Bitboard = Bitboard(0x7F7F7F7F7F7F7F7F);
pub const NOT_AB_FILE: Bitboard = Bitboard(0xFCFCFCFCFCFCFCFC);
pub const NOT_GH_FILE: Bitboard = Bitboard(0x3F3F3F3F3F3F3F3F);
use std::mem::transmute;
use crate::board::square::Square::A1;

pub enum SquareColor {
    White,
    Black,
}

/// Helper enum representing each square on the chess board.
/// Note that each member's value corresponds with bit index,
/// as defined in Little Endian Rank-File Mapping (the least significant bit is A1, then A2 and so on).
///
///   v-a8 (bit 56)
///   0 0 0 0 0 0 0 0 <- h8 (bit 63)
///   0 0 0 0 0 0 0 0
///   0 0 0 0 0 0 0 0
///   0 0 0 0 0 0 0 0
///   0 0 0 0 1 0 0 0
///   0 0 0 0 0 0 0 0 <- h3 (bit 23)
///   0 0 0 0 0 0 0 0
///   0 0 0 0 0 0 0 0 <- h1 (bit 7)
///   ^-a1 (bit 0)

#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum Square {
    A1 = 0,
    B1,
    C1,
    D1,
    E1,
    F1,
    G1,
    H1,
    A2,
    B2,
    C2,
    D2,
    E2,
    F2,
    G2,
    H2,
    A3,
    B3,
    C3,
    D3,
    E3,
    F3,
    G3,
    H3,
    A4,
    B4,
    C4,
    D4,
    E4,
    F4,
    G4,
    H4,
    A5,
    B5,
    C5,
    D5,
    E5,
    F5,
    G5,
    H5,
    A6,
    B6,
    C6,
    D6,
    E6,
    F6,
    G6,
    H6,
    A7,
    B7,
    C7,
    D7,
    E7,
    F7,
    G7,
    H7,
    A8,
    B8,
    C8,
    D8,
    E8,
    F8,
    G8,
    H8,
}

impl Square {
    /// Returns index for given square (0 - 63).
    pub fn as_index(self) -> usize {
        return self as usize;
    }

    /// Returns bit index for given square (0 - 63).
    pub fn as_bit_index(self) -> u8 {
        return self.as_index() as u8;
    }

    /// Returns a Square enum member based on the board coords.
    pub fn from_board_coords(file: u8, rank: u8) -> Square {
        if rank >= 8 || file >= 8 {
            panic!("Incorrect rank or file: {} | {}", rank, file);
        }

        let index = rank * 8 + file;

        return unsafe { transmute(index) };
    }

    /// Returns board coords for given Square, in a (file, rank) tuple.
    pub fn to_board_coords(self) -> (u8, u8) {
        // Equivalent of mod 8.
        let file = (self as u8) & 0b111;

        // Equivalent of div 8.
        let rank = (self as u8) >> 3;

        return (file, rank);
    }
}

pub const SQUARES: [Square; 64] = [
    Square::A1, Square::B1, Square::C1, Square::D1, Square::E1, Square::F1, Square::G1, Square::H1,
    Square::A2, Square::B2, Square::C2, Square::D2, Square::E2, Square::F2, Square::G2, Square::H2,
    Square::A3, Square::B3, Square::C3, Square::D3, Square::E3, Square::F3, Square::G3, Square::H3,
    Square::A4, Square::B4, Square::C4, Square::D4, Square::E4, Square::F4, Square::G4, Square::H4,
    Square::A5, Square::B5, Square::C5, Square::D5, Square::E5, Square::F5, Square::G5, Square::H5,
    Square::A6, Square::B6, Square::C6, Square::D6, Square::E6, Square::F6, Square::G6, Square::H6,
    Square::A7, Square::B7, Square::C7, Square::D7, Square::E7, Square::F7, Square::G7, Square::H7,
    Square::A8, Square::B8, Square::C8, Square::D8, Square::E8, Square::F8, Square::G8, Square::H8,
];
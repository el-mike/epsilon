use crate::bitboard::bitboard::{Bitboard};
use std::fmt;
use std::fmt::Formatter;
use crate::board::square::Square;

impl fmt::Display for Bitboard {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        /*
            v-a8 (bit 56)
            0 0 0 0 0 0 0 0 <- h8 (bit 63)
            0 0 0 0 0 0 0 0
            0 0 0 0 0 0 0 0
            0 0 0 0 0 0 0 0
            0 0 0 0 1 0 0 0
            0 0 0 0 0 0 0 0 <- h3 (bit 23)
            0 0 0 0 0 0 0 0
            0 0 0 0 0 0 0 0 <- h1 (bit 7)
            ^-a1 (bit 0)
        */
        let mut result = String::new();

        let mut file: u8 = 0;
        let mut rank: u8 = 7;

        loop {
            let bit_index: u8 = rank * Bitboard::WIDTH + file;
            let char = if self.is_set_at(bit_index) { '1' } else { '0' };

            result.push_str(format!(" {} ", char).as_str());

            file += 1;

            if file == Bitboard::WIDTH {
                result.push('\n');
                file = 0;

                if rank != 0 {
                    rank -= 1
                } else {
                    break;
                };
            }
        }

        return write!(f, "{}", result);
    }
}


pub fn render_moves_for_piece(piece_square: Square, attacks_bitboard: Bitboard) {
    let mut result = String::new();

    let mut file: u8 = 0;
    let mut rank: u8 = 7;

    loop {
        let bit_index: u8 = rank * Bitboard::WIDTH + file;
        let mut char;

        if bit_index == piece_square.as_bit_index() {
            char = '*';
        } else {
            char = if attacks_bitboard.is_set_at(bit_index) { '1' } else { '_' };
        }

        result.push_str(format!(" {} ", char).as_str());

        file += 1;

        if file == Bitboard::WIDTH {
            result.push('\n');
            file = 0;

            if rank != 0 {
                rank -= 1
            } else {
                break;
            };
        }
    }

    println!("{}", result);
}
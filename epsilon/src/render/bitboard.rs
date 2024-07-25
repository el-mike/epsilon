use crate::bitboard::bitboard::{Bitboard, BITBOARD_WIDTH};
use std::fmt;
use std::fmt::Formatter;

impl fmt::Display for Bitboard {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut result = String::new();

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

        let mut file: u8 = 0;
        let mut rank: u8 = 7;

        loop {
            let bit_index: u8 = rank * BITBOARD_WIDTH + file;
            let char = if self.is_set_at(bit_index) { '1' } else { '0' };

            result.push_str(format!(" {} ", char).as_str());

            file += 1;

            if file == BITBOARD_WIDTH {
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

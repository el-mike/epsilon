use crate::board::bitboard::Bitboard;

/// Returns a mask with a bit set specified by bit_index (nth bit of the number).
pub fn get_bitmask_for_index(bit_index: u8) -> u64 {
    return 1_u64 << bit_index;
}
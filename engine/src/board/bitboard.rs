use crate::common::coord::Coord;

const UNIVERSE: u64 = 0xffffffffffffffff;

pub type Bitboard = u64;

pub fn is_empty(bitboard: Bitboard) -> bool {
    return bitboard == 0;
}

pub fn is_universal(bitboard: Bitboard) -> bool {
    return bitboard == UNIVERSE;
}

pub fn is_set_at(bitboard: Bitboard, bit_index: u64) -> bool {
    return (bitboard & (1_u64 << bit_index)) != 0;
}


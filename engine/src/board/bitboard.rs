use std::ops::{BitAnd, BitOr, BitXor, BitAndAssign, BitOrAssign, BitXorAssign, Not, Deref};
use crate::common::math::get_bitmask_for_index;

const UNIVERSE: u64 = 0xffffffffffffffff;
pub const BITBOARD_WIDTH: u8 = 8;
pub const BITBOARD_HEIGHT: u8 = 8;

/// Returns a bitmask as a Bitboard instance.
fn get_bitboard_for_index(bit_index: u8) -> Bitboard {
    return Bitboard(get_bitmask_for_index(bit_index))
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub struct Bitboard(pub u64);

impl Bitboard {
    /// Returns true if bitboard has no bits set, false otherwise.
    pub fn is_empty(self) -> bool {
        return self == Bitboard(0);
    }

    /// Returns true if bitboard has all bits set, false otherwise.
    pub fn is_universal(self) -> bool {
        return self == Bitboard(UNIVERSE);
    }

    /// Returns true if bitboard has a bit at given position, false otherwise.
    pub fn is_set_at(self, bit_index: u8) -> bool {
        return (self & get_bitboard_for_index(bit_index)) != Bitboard(0);
    }

    /// Sets a bit in Bitboard to 1 ato given position.
    pub fn set_at(&mut self, bit_index: u8) {
        *self |= get_bitboard_for_index(bit_index);
    }
}

impl Deref for Bitboard {
    type Target = u64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl BitAnd for Bitboard {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        return Self(self.0 & rhs.0);
    }
}

impl BitOr for Bitboard {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        return Self(self.0 | rhs.0);
    }
}

impl BitXor for Bitboard {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        return Self(self.0 ^ rhs.0);
    }
}

impl BitAndAssign for Bitboard {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = Self(self.0 & rhs.0);
    }
}

impl BitOrAssign for Bitboard {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = Self(self.0 | rhs.0);
    }
}

impl BitXorAssign for Bitboard {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = Self(self.0 ^ rhs.0);
    }
}

impl Not for Bitboard {
    type Output = Self;

    fn not(self) -> Self::Output {
        return Self(!self.0);
    }
}

// type BB = u64;
//
// impl BitAnd for BB {
//     type Output = Self;
//
//     fn bitand(self, rhs: Self) -> Self::Output {
//         return
//     }
// }
//
// fn test(l: BB, r: BB) -> BB {
//     return l & r;
// }
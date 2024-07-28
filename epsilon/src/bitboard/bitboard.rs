use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Shl, Shr};

const UNIVERSE: u64 = 0xffffffffffffffff;

/// Returns a mask with a bit set specified by bit_index (nth bit of the number).
pub fn get_bitmask_for_index(bit_index: u8) -> u64 {
    return 1_u64 << bit_index;
}

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub struct Bitboard(pub u64);

impl Bitboard {
    pub const WIDTH: u8 = 8;
    pub const SIZE: u8 = 64;

    /// Returns an empty Bitboard.
    pub fn get_empty() -> Self {
        return Bitboard(0);
    }

    /// Returns Bitboard for given bit index.
    pub fn from_bit_index(bit_index: u8) -> Self {
        return Self(get_bitmask_for_index(bit_index));
    }

    /// Returns true if bitboard has no bits set, false otherwise.
    pub fn is_empty(&self) -> bool {
        return self.0 == 0;
    }

    /// Returns true if bitboard has all bits set, false otherwise.
    pub fn is_universal(&self) -> bool {
        return self.0 == UNIVERSE;
    }

    /// Returns true if bitboard has a bit at given position, false otherwise.
    pub fn is_set_at(&self, bit_index: u8) -> bool {
        return (self.0 & get_bitmask_for_index(bit_index)) != 0;
    }

    /// Sets a bit in Bitboard  at given position, and returns new Bitboard.
    pub fn set_at(self, bit_index: u8) -> Self {
        return Self(self.0 | get_bitmask_for_index(bit_index));
    }

    /// Unsets a bit in Bitboard at given position, and returns new Bitboard.
    pub fn unset_at(self, bit_index: u8) -> Self {
        return Self(self.0 & !get_bitmask_for_index(bit_index));
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

impl Shl<u8> for Bitboard {
    type Output = Self;

    fn shl(self, rhs: u8) -> Self::Output {
        return Self(self.0 << rhs);
    }
}

impl Shr<u8> for Bitboard {
    type Output = Self;

    fn shr(self, rhs: u8) -> Self::Output {
        return Self(self.0 >> rhs);
    }
}
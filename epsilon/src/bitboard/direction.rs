/// All the following directions apply to Little Endian File-Rank mapping.
/// Directions follow this compass rose:
///   noWe         nort         noEa
///           +7    +8    +9
///               \  |  /
///   west    -1 --  0 -- +1    east
///               /  |  \
///           -9    -8    -7
///   soWe         sout         soEa
/// Note that DirectionValue is always positive, representing absolute value -
/// the actual direction applied should be decided at calling, by using right of left bit shift.

pub struct Direction {}

impl Direction {
    pub const NORTH_WEST: u8 = 7;
    pub const NORTH: u8 = 8;
    pub const NORTH_EAST: u8 = 9;
    pub const EAST: u8 = 1;
    pub const SOUTH_EAST: u8 = 7;
    pub const SOUTH: u8 = 8;
    pub const SOUTH_WEST: u8 = 9;
    pub const WEST: u8 = 1;
}

pub struct KnightDirection {}

impl KnightDirection {}
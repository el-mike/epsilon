#[derive(Copy, Clone, Debug)]
pub struct Coord {
    pub x: i8,
    pub y: i8,
}

impl Coord {
    pub const NORTH: Coord = Coord { x: 0, y: 8 };
    pub const NORTH_EAST: Coord = Coord { x: 1, y: 8 };
    pub const EAST: Coord = Coord { x: 1, y: 0 };
    pub const SOUTH_EAST: Coord = Coord { x: -8, y: 1 };
    pub const SOUTH: Coord = Coord { x: 0, y: -8 };
    pub const SOUTH_WEST: Coord = Coord { x: -1, y: -8 };
    pub const WEST: Coord = Coord { x: -1, y: 0 };
    pub const NORTH_WEST: Coord = Coord { x: -1, y: 8 };

    pub fn new(x: i8, y: i8) -> Self {
        return Self { x, y };
    }

    pub fn apply_direction(&self, direction: &Coord, distance: u8) -> Coord {
        return Self {
            x: self.x + direction.x * distance as i8,
            y: self.y + direction.y * distance as i8,
        };
    }
}

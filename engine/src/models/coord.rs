pub struct Coord {
    pub x: u8,
    pub y: u8,
}

impl Coord {
    pub fn new(x: u8, y: u8) -> Self {
        return Self {
            x, y,
        };
    }
}

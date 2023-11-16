use crate::common::coord::Coord;

pub struct AlgebraicCoord {
    value: String,
}

const ASCII_A_CODE: i8 = 97;

impl AlgebraicCoord {
    pub fn from_string(algebraic_coord: &str) -> Self {
        let chars: Vec<char> = algebraic_coord.chars().collect();

        if chars.len() != 2 {
            panic!("Incorrect algebraic coord: {}", algebraic_coord);
        }

        return Self { value: String::from(algebraic_coord) };
    }

    pub fn from_coord(coord: &Coord) -> Self {
        let rank = char::from(coord.y as u8);

        let ascii_file = ASCII_A_CODE + coord.x;
        let file = (ascii_file as u8) as char;

        return Self {
            value: format!("{file}{rank}"),
        }
    }

    pub fn to_coord(&self) -> Coord {
        let chars: Vec<char> = self.value.chars().collect();

        if chars.len() != 2 {
            panic!("Malformed algebraic coord: {}", self.value);
        }

        /// We need to add "1" to the file, as subtracting ASCII "a" code is zero-based.
        let file = ((chars[0] as u32) - ASCII_A_CODE as u32) + 1;
        let rank = chars[1].to_digit(10).unwrap();

        return Coord {
            x: file as i8,
            y: rank as i8,
        }
    }
}

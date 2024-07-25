use crate::board::square::Square;

pub struct AlgebraicCoord {
    value: String,
}

const ASCII_A_CODE: u8 = 97;

impl AlgebraicCoord {
    pub fn from_string(algebraic_coord: &str) -> Self {
        let chars: Vec<char> = algebraic_coord.chars().collect();

        if chars.len() != 2 {
            panic!("Incorrect algebraic coord: {}", algebraic_coord);
        }

        return Self {
            value: String::from(algebraic_coord),
        };
    }

    pub fn from_square(square: Square) -> Self {
        let (file, rank) = square.to_board_coords();

        let ascii_file = ASCII_A_CODE + file;
        let file_char = ascii_file as char;

        let rank_char = char::from(rank);

        return Self {
            value: format!("{file_char}{rank_char}"),
        };
    }

    pub fn to_square(&self) -> Square {
        let chars: Vec<char> = self.value.chars().collect();

        if chars.len() != 2 {
            panic!("Malformed algebraic coord: {}", self.value);
        }

        /// We need to add "1" to the file, as subtracting ASCII "a" code is zero-based.
        let file = (chars[0] as u8 - ASCII_A_CODE) + 1;
        let rank: u8 = chars[1].to_digit(10).unwrap().try_into().unwrap();

        return Square::from_board_coords(file, rank);
    }
}

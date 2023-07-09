use std::fmt;

use crate::common::coord::Coord;
use crate::board::board::{Board, BOARD_WIDTH};

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::new();

        let mut file = 0;
        let mut rank = 7;

        loop {
            let piece = self.get_piece(&Coord::new(file, rank));

            result.push_str(format!(" {} ", piece).as_str());

            file += 1;

            if file == BOARD_WIDTH {
                result.push('\n');

                file = 0;

                if rank != 0 {
                    rank -= 1;
                } else {
                    break;
                }
            }
        }

        write!(f, "{}", result)
    }
}

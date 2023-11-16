use crate::common::coord::Coord;

pub fn out_of_bounds_index_message(coord: &Coord) -> String {
    return format!("Out of bounds coord: x = {}, y = {}", coord.x, coord.y);
}

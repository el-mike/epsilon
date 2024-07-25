extern crate core;

mod board;
mod fen;
mod moves;
mod position;
mod render;

#[no_mangle]
pub extern "C" fn evaluate() -> String {
    return String::from("e2e4");
}

#[no_mangle]
pub extern "C" fn get_available_moves() -> String {
    return String::from("e2,e4");
}

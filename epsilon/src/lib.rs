extern crate core;

use std::ffi::{c_char, CString};

mod bitboard;
mod board;
mod fen;
mod moves;
mod position;
mod render;

#[no_mangle]
pub extern "C" fn evaluate() -> *const c_char {
    return CString::new("e2e4").unwrap().as_ptr();
}

#[no_mangle]
pub extern "C" fn get_available_moves() -> *const c_char {
    let s = CString::new("e2,e4").unwrap();
    let p = s.as_ptr();
    std::mem::forget(s);

    return p;
}
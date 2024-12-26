// File: lib.rs

use std::os::raw::c_int;

#[no_mangle]
pub extern "C" fn add(a: c_int, b: c_int) -> c_int {
    a + b
}

#[no_mangle]
pub extern "C" fn subtract(a: c_int, b: c_int) -> c_int {
    a - b
}

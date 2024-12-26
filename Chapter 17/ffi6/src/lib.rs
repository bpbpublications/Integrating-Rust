// File: lib.rs

use std::os::raw::c_int;

#[no_mangle]
pub extern "C" fn process_array(input: *const c_int, len: usize) -> c_int {
    // Process the array (return the sum of its elements in this example)
    let slices = unsafe { std::slice::from_raw_parts(input, len) };
    slices.iter().sum()
}

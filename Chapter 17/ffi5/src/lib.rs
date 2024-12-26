// File: lib.rs

use std::os::raw::c_int;

#[repr(C)]
pub struct MyStruct {
    pub x: c_int,
    pub y: c_int,
}

#[no_mangle]
pub extern "C" fn process_struct(input: MyStruct) -> c_int {
    // Process the struct (return the sum of x and y in this example)
    input.x + input.y
}

// File: lib.rs

use std::ffi::CString;
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn process_string(input: *const c_char) -> *mut c_char {
    // Convert the input C-style string to a Rust CString
    let input_str = unsafe { CString::from_raw(input as *mut c_char) };

    // Extract the original string from the CString
    let original_string = input_str.to_str().unwrap();
    
    // Process the string (reverse it in this example)
    let processed_string = original_string.chars().rev().collect::<String>();
    
    // Convert the processed string to a CString and return the raw pointer
    CString::new(processed_string).unwrap().into_raw()
}

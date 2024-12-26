// File: lib.rs

use std::os::raw::c_int;

#[no_mangle]
pub extern "C" fn safe_divide(dividend: c_int, divisor: c_int) -> Result<c_int, &'static str> {
    if divisor == 0 {
        Err("Division by zero is not allowed")
    } else {
        Ok(dividend / divisor)
    }
}

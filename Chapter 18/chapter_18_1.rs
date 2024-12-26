// Without HAL (Direct register manipulation)
unsafe fn toggle_gpio_pin_direct() {
    // Assuming a hypothetical microcontroller register for GPIO
    let gpio_register = 0x4000_0000 as *mut u32;

    // Toggle the GPIO pin
    *gpio_register ^= 1 << 5;
}

// With HAL (Abstraction)
fn toggle_gpio_pin_with_hal(hal: &HAL) {
    // Use HAL function to toggle GPIO pin
    hal.toggle_gpio_pin(5);
}

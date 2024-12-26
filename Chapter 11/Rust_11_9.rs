fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        return Err("Division by zero!".to_string());
    }

    Ok(a / b)
}

fn main() {
    let dividend = 10;
    let divisor = 2;

    match divide(dividend, divisor) {
        Ok(result) => println!("Result: {}", result),
        Err(err) => eprintln!("Error: {}", err),
    }
}

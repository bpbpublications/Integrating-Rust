fn main() {
    let point = (3, 4);

    match point {
        (0, 0) => println!("origin"),
        (x, 0) => println!("x-axis, x = {}", x),
        (0, y) => println!("y-axis, y = {}", y),
        (x, y) => println!("({}, {})", x, y),
    }
}

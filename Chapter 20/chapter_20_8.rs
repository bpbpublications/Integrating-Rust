fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

fn main() {
    let result = add(10, 20); // Positional arguments
    let result = add(b = 20, a = 10); // Named arguments
}

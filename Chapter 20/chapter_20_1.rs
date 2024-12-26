fn main() {
    let x = 10;
    {
        let x = 20; // Shadowing the outer variable x
        println!("Inner x: {}", x); // Output: Inner x: 20
    }
    println!("Outer x: {}", x); // Output: Outer x: 10
}

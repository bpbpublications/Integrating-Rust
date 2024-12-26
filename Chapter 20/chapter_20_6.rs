fn main() {
    let mut num = 0;
    loop {
        println!("{}", num); // Prints numbers indefinitely
        num += 1;
        if num == 5 {
            break; // Terminates the loop when num reaches 5
        }
    }
}

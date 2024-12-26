fn main() {
    let mut counter = 0;
    while counter < 10 {
        if counter == 5 {
            break; // Exits the loop when counter is 5
        }
        if counter % 2 == 0 {
            continue; // Skips even numbers
        }
        println!("{}", counter);
        counter += 1;
    }
}

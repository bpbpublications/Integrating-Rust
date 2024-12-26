fn main() {
    let text = "Hello\nWorld\n!";
    for line in text.lines() {
        println!("{}", line);
    }
}
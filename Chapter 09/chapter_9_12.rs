fn main() {
    let sentence = "The quick brown fox";
    let words: Vec<&str> = sentence.split(" ").collect();
    for w in words{
        println!("{}", w);
    }
}

fn main() {
    let s1 = String::from("Hello");
    let s2 = String::from("world!");
    
    let s3 = format!("{} {}", s1, s2); // s1 and s2 can still be used
    
    println!("{}", s3);
}

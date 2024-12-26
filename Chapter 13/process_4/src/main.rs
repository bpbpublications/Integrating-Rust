use std::env;

fn main() {
    // Get the value of the HOME environment variable
    if let Some(home_dir) = env::var("HOME").ok() {
        println!("Home directory: {}", home_dir);
    }

    // Set a new environment variable
    env::set_var("MY_VARIABLE", "Hello, Rust!");

    // Uncomment the following lines to print the new environment variable.
    // if let Some(my_var) = env::var("MY_VARIABLE").ok() {
    // println!(“MY_VARIABLE: {}", my_var);
    //}
}

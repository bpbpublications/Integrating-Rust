use std::fs;

fn main() {
    let result = fs::create_dir("example_dir");

    match result {
        Ok(()) => println!("Directory created successfully!"),
        Err(e) => println!("Error creating directory: {:?}", e),
    }
}

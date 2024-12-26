#[allow(dead_code)]
fn main() {
    enum Direction {
        North,
        South,
        East,
        West,
    }
    
    let direction = Direction::North;
    
    match direction {
        Direction::North => println!("heading north"),
        Direction::South => println!("heading south"),
        Direction::East => println!("heading east"),
        Direction::West => println!("heading west"),
    }    
}

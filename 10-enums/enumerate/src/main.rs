enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn main() {
    let player_direction: Direction = Direction::Up; 
    println!("Hello, player!");

    match player_direction {
        Direction::Up => println!("Player goes up!"),
        Direction::Down => println!("Player goes down!"),
        Direction::Left => println!("Player goes left!"),
        Direction::Right => println!("Player goes right!"),
    }
}

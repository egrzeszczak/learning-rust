use std::io;

fn main() {
    let mut input = String::new();

    match io::stdin().read_line(&mut input) {
        Ok(_) => {},
        Err(e) => { println!("Error reading input: {}", e)}
    }
}

use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::create("output.txt")
        .expect("Can't create file");

    file.write_all(b"Welcome to dcode!")
        .expect("Can't write to file");
}

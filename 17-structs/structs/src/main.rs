struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

fn main() {
    let background_color = Color {
        red: 255,
        green: 50,
        blue: 145
    };

    println!("{}, {}, {}", background_color.red, background_color.green, background_color.blue);
}

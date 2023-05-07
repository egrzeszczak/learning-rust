struct Color {
    red: u8,
    green: u8,
    blue: u8
}

fn print_color(color: &Color) {
    println!("{}, {}, {}", color.red, color.green, color.blue);
}

fn main() {
    let background_color = Color { red: 255, green: 0, blue: 0};
    print_color(&background_color);
}


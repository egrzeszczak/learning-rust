struct Color(u8, u8, u8);

fn main() {
    let mut foreground_color = Color(123,67,23);

    foreground_color.2 = 30;

    println!("{}, {}, {}", foreground_color.0, foreground_color.1, foreground_color.2);
}

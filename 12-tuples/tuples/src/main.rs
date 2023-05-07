fn main() {
    let uno: (u8, u8, u8) = (5, 10, 15);

    let due = ("Test", '\\', 14.7, (1, 3.5));
    println!("{}", due.1);

    let (_uno_a, _uno_b, _uno_c) = uno;
    let (_due_a, _due_b, _due_c, _due_d) = due;
}

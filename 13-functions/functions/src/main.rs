fn is_divisible_by_6(num: u64) -> bool {
    return num % 6 == 0;
}

fn print_to_number(num: u64) {
    for n in 1..num {
        println!("{}", n);
        if is_divisible_by_6(n) {
            println!("YAY!")
        }
    }
}


fn main() {
    let num: u64 = 32;
    print_to_number(num);
}
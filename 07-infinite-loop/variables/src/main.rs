fn main() {
    let mut n: i32 = 0;

    loop {
        n += 1;

        if n > 700000 && n < 800000  {
            continue;
        }

        if n > 1000000 {
            break;
        }

        println!("{}", n)
    }
}

fn main() {
    let x = 10;

    {
        let x = 15;
    }

    println!("{}", x);
}

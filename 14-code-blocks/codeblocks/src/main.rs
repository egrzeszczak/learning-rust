fn main() {
    let x = 10;

    {
        let y = 5;
        println!("{}", x);
        println!("{}", y); // OK 
    }

    println!("{}", x);
    // println!("{}", y); // ERR: Out of scope 
}

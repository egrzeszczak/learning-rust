fn main() {
    // All variables are immutable by default
    // ERR: let x = 45; 
    let mut x = 45;

    println!("The value of x is {}", x);
    
    x = 60;
    
    println!("The value of x is now {}", x);
}

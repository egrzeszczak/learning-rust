fn main() {
//     let a = 45; // i32
//     let b: i64 = 45; // i64
//     let c: u32 = 45; // unsigned i32
//     let d: u64 = 45; // unsigned i64
//     let e = 3.14 // floating point (f32)
//     let f: f64 = 3.14 // floating point (f64)
//     let g: bool = false // boolean

    let n: i32 = 45;

    if n < 30 {
        println!("The number is less than 30");
    } else {
        println!("The number is greater or equal 30")
    }

    if n == 45 {
        println!("The number is 45")
    }
    if n != 45 {
        println!("The number is not 45")
    } 
    else if n == 50 {
        println!("The number is 50")
    }

    
}

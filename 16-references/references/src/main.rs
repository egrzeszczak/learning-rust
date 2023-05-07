fn main() {
    let mut x: i32 = 10;
    {
        let xr: &mut i32= &mut x;
        *xr = 155;
    }
    println!("{}", x);
}

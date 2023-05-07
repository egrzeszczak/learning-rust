fn main() {

    let numbers: [i32; 5] = [1, 2, 3, 4, 5];

    println!("{:#?}", numbers);

    for n in numbers.iter() {
        println!("{}", n);
    }

    for i in 0..numbers.len() {
        println!("{}", numbers[i]);
    }
    
}

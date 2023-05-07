fn main() {
    
    // let my_vector: Vec<i32> = Vec::new();
    let mut my_vector = vec![1, 2, 3, 4, 5];

    // push
    my_vector.push(15);

    // remove
    my_vector.remove(2);

    println!("{:#?}", my_vector);
}

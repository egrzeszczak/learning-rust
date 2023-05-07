fn main() {
    // for i in 1..11 {
    //     println!("{}", i)
    // }

    let _nms = 30..61;
    // for n in nms {
    //     println!("{}", n);
    // }

    let animals = vec!["Rabbit", "Dog", "Cat"];
    for animal in animals.iter() {
        println!("The animal is {}", animal);
    }
    for (index, animal) in animals.iter().enumerate() {
        println!("The animal no.{} is {}", index, animal);
    }
}

fn main() {
    let my_string = String::from("The quick brown fox jumps over the lazy dog");
    
    // Length
    println!("Length: {}", my_string.len());

    // Is empty? 
    println!("Is empty?: {}", my_string.is_empty());
    
    // Split
    for word in my_string.split_whitespace() {
        println!("{}", word);
    }
    // +
    // println!("{:#?}", my_string.split_whitespace());

    // Contains
    println!("Contains 'fox'?: {}", my_string.contains("fox"));
    
    // Push to string
    // my_string.push_str(" ';-- "); ERR: String needs to be mutable
    // let mut my_string = String::from("The quick brown fox jumps over the lazy dog");
}

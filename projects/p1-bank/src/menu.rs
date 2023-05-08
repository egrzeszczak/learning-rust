use std::clone::Clone;
use crate::action;


#[derive(Clone)]
pub struct Option {
    pub display_name: String,
    pub name: String,
    pub keys: Vec<char>,
    pub action: action::Action,
}
impl Option {
    pub fn print(&self) {
        println!("[{}] {} ({})", self.keys[0], self.display_name, self.name)
    }
}



pub fn display_options(options: &Vec<Option>) {
    for opt in options {
        opt.print();
    }
}


pub fn read_choice(options: &Vec<Option>) -> Option {
    let mut input = String::new();
    match std::io::stdin().read_line(&mut input) {
        Ok(_) => {},
        Err(e) => {println!("Error reading input: {}", e)}
    }

    for opt in options {
        if opt.keys.iter().any(|&i| i==input.trim().chars().next().expect("string is empty")) {
            return opt.clone();
        }
    }

    return Option {
        display_name: String::from("null"),
        name: String::from("null"),
        keys: vec!['0'],
        action: action::Action::None,
    };
}
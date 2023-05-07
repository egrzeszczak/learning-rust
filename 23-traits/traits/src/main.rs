struct User {
    username: String,
    password: String
}

impl ToString for User {
    fn to_string(&self) -> String {
        return format!("username={}, password={}", self.username, self.password);
    }
}

fn main() {
    let user1 = User {
        username: String::from("egrzeszczak"),
        password: String::from("egrzeszczak123!@#")
    };

    println!("{}", user1.to_string());
}

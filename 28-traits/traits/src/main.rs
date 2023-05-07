struct User {
    username: String,
    password: String,
    email: String,
    attributes: Vec<String>,
}

impl User {
    fn to_string(&self) {
        println!("username={} password={} passcompl={} email={} attributes={:?}",
            self.username,
            self.password,
            self.check_password_compliance(),
            self.email,
            self.attributes,
        )
    }
}

trait PasswordComplexityPolicy {
    // Min. password length
    fn check_password_compliance(&self) -> bool;
}

impl PasswordComplexityPolicy for User {
    fn check_password_compliance(&self) -> bool { 
        return self.password.len() > 6;
    }
}

fn main() {
    let user = User {
        username: String::from("egrzeszczak"),
        password: String::from("test123!@#"),
        email: String::from("egrzeszczak@rust.learn"),
        attributes: vec![String::from("global-admin"), String::from("protected-user")]
    };

    user.to_string();

    println!("{}", user.check_password_compliance());
}

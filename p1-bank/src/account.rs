pub struct Account {
    pub name: String,
    pub value: f32,
}

pub fn create_account(accounts: &mut Vec<Account>) {
    accounts.push(Account {
        name: String::from("New Account"),
        value: 0.0,
    });
}

pub fn get_all_accounts(accounts: &Vec<Account>) {
    println!("Name (Value)\n-------------------\n");
    for acc in accounts {
        println!("{} ({})", acc.name, acc.value);
    }
    println!("\n\n\n")
}
mod account;
mod menu;
mod action;


fn main() {
    println!("bank v0.1.0 by egrzeszczak");
    print!("{}[2J", 27 as char);

    let mut accounts: Vec<account::Account> = vec![];

    let options: Vec<menu::Option> = vec![
        menu::Option {
            display_name: String::from("Create an account"),
            name: String::from("create-account"),
            keys: vec!['C', 'c'],
            action: action::Action::CreateAccount,
        },
        menu::Option {
            display_name: String::from("Display existing accounts"),
            name: String::from("display-existing"),
            keys: vec!['D', 'd'],
            action: action::Action::GetAllAccounts,
        },
        menu::Option {
            display_name: String::from("Quit"),
            name: String::from("quit"),
            keys: vec!['Q', 'q'],
            action: action::Action::ExitProgram,
        }
    ];

    loop {
        // Display options
        menu::display_options(&options);

        // Read user choice
        let choice: menu::Option = menu::read_choice(&options); 
        
        // Actions
        match choice.action {
            action::Action::CreateAccount => { print!("{}[2J", 27 as char); account::create_account(&mut accounts) },
            action::Action::GetAllAccounts => { print!("{}[2J", 27 as char); account::get_all_accounts(&accounts) },
            action::Action::ExitProgram => break,
            _ => continue,
        }

    }
}
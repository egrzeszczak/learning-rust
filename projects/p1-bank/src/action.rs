#[derive(Clone)]



pub enum Action {
    CreateAccount, // Utwórz konto
    GetAllAccounts, // Wyświetl wszystkie konta
    ExitProgram, // Zakończ program
    None, // Nic nie rób (domyślnie)
}
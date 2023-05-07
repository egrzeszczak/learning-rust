enum Status {
    New,
    Accepted,
    InProgress,
    OnHold,
    Done,
    Closed,
    Denied,
}

struct Ticket {
    title: String,
    description: String,
    status: Status,
}

impl Ticket {
    fn to_string(&self) -> String {
        return format!("title='{}' description='{}' status='{}' active='{}'", self.title, self.description, self.status_string(), self.is_active()) 
    }

    fn status_string(&self) -> String {
        match self.status {
            Status::New => return String::from("new"),
            Status::Accepted => return String::from("accepted"),
            Status::InProgress => return String::from("in-progress"),
            Status::OnHold => return String::from("on-hold"),
            Status::Done => return String::from("done"),
            Status::Closed => return String::from("closed"),
            Status::Denied => return String::from("denied"),
        }
    }

    fn is_active(&self) -> bool {
        match self.status {
            Status::New | Status::Accepted | Status::InProgress | Status::OnHold => return true, 
            Status::Done | Status::Closed | Status::Denied => return false,
        }
    }
}



fn main() {
    let ticket1 = Ticket {
        title: String::from("Test ticket"),
        description: String::from("This is a test ticket for testing purposes"),
        status: Status::OnHold,
    };
    println!("{}", ticket1.to_string());
}

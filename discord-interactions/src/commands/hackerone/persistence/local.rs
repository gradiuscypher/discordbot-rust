pub struct Persistence {
    last_ticket_number: Option<String>,
}

impl Persistence {
    pub fn last_ticket_number(&mut self, ticket_number: Option<String>) -> Option<String> {
        match ticket_number {
            Some(ticket_number) => {
                self.last_ticket_number = Some(ticket_number);
                self.last_ticket_number.clone()
            }
            None => self.last_ticket_number.clone(),
        }
    }
}

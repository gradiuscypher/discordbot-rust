use serde::Deserialize;
use std::path::Path;
use std::{fs::File, io::ErrorKind};

#[derive(Debug, Deserialize)]
struct Persistence {
    last_ticket_number: String,
}

pub fn last_ticket_number(ticket_number: Option<String>) -> Option<String> {
    let file_path = Path::new("local_persistence.json");
    let file = File::open(file_path);

    let file = match file {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::options()
                .write(true)
                .read(true)
                .create(true)
                .open(file_path)
            {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };

    let persistence: Persistence = serde_json::from_reader(file).expect("error while reading");
    println!("{:?}", persistence);
    Some("bob".to_owned())

    // match ticket_number {
    //     Some(ticket_number) => Some("bob".to_owned()),
    //     None => None,
    // }
}

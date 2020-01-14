pub mod structs;
use crate::structs::class;
use crate::structs::data_connection::{DatabaseConnection, DbMessage};
use serde_json::Value;
use std::sync::mpsc::Sender;

use std::io;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub fn get_database_connection(
    message_channel: Sender<Value>,
) -> (DatabaseConnection, Sender<Value>) {
    DatabaseConnection::new(message_channel)
}

pub fn run_loop() {
    let mut guess = String::new();

    loop {
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");
        let message = DbMessage::new(guess.trim().to_string().clone());
        println!("{:#?}", message);
        let exit_command = String::from("exit_application");
        let msg_string = message.to_string();

        match msg_string.trim() {
            "exit_application" => break,
            _ => break, //println!("{:#?}", message),
        };
        // guess.clear();
    }
}

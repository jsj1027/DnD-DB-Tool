pub mod structs;
use crate::structs::class;
use crate::structs::data_connection::DatabaseConnection;
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
        io::stdin().read_line(&mut guess)
            .expect("failed to read line");    
        match guess.trim(){
            "exit" => break,
            _ => println!("{:#?}", guess.trim()),
        };
        guess.clear();
    }
}

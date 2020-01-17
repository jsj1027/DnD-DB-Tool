pub mod structs;
use crate::structs::data_connection::{DatabaseConnection, DbMessage};
use serde_json::Value;
use std::io;
use std::sync::mpsc::Sender;

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

pub fn run_loop(send_message_channel: Sender<Value>) {
    let mut guess = String::new();
    loop {
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");
        let message = DbMessage::new(guess.trim().to_string());
        let message_json = serde_json::to_value(message).unwrap();
        send_message_channel.send(message_json);
        // let exit_command = String::from("exit_application");

        // match msg_string.trim() {
        //     "exit_application" => break,
        //     _ => println!("{:#?}", message),
        // };
        guess.clear();
    }
}

pub mod structs;
use crate::structs::class;
use crate::structs::data_connection::DatabaseConnection;
use std::sync::mpsc::Sender;
use serde_json::Value;

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

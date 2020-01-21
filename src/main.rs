mod structs;

use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

use rusqlite::{Connection, Error};

use serde_json::Value;

use crate::structs::race::Race;
use db_lib;
use db_lib::structs::data_connection::{DatabaseConnection, DbMessage};

fn main() {
    // let (send_channel, receive_channel): (Sender<Value>, Receiver<Value>) = mpsc::channel();

    // let connections = setup_database_thread(send_channel.clone());
    // let database_thread = connections.0;

    // let send_db_message_channel = connections.1;
    // db_lib::run_loop(send_db_message_channel);

    // match character {
    //     "bard" | "Bard" | "BARD" => {
    //         println!("we gonna create a bard");
    //         let bard = DbMessage {
    //             action: String::from("create"),
    //             verb: String::from("bard"),
    //             item: Option::None,
    //         };
    //         let result = send_db_message_channel.send(serde_json::to_value(bard).unwrap());
    //         match result {
    //             Ok(()) => println!("creating bard"),
    //             Err(error) => {
    //                 println!("{}", error);
    //             }
    //         }
    //     }
    //     _ => println!("Not a creatable class."),
    // }
    let connection = DatabaseConnection::new_test();
    let race_name = "Dwarf";
    Race::new(&connection, &race_name);
    let random_race = Race::random(&connection);
    println!("{:#?}", random_race);
}

// fn setup_database_thread(
//     sender_channel: Sender<Value>,
// ) -> (std::thread::JoinHandle<()>, Sender<Value>) {
//     let connection: (DatabaseConnection, Sender<Value>) =
//         db_lib::get_database_connection(sender_channel);

//     let database = connection.0;
//     let database_send_message_channel = connection.1;

//     let database_thread = thread::spawn(move || database.run());

//     (database_thread, database_send_message_channel)
// }

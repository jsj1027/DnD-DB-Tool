#[macro_use]
extern crate clap;
use clap::App;

mod structs;

use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

use rusqlite::{Connection, Error};

use serde_json::Value;

use db_lib;
use db_lib::structs::data_connection::{DatabaseConnection, DbMessage};

fn main() {
    let yaml = load_yaml!("configuration.yaml");
    let matches = App::from(yaml).get_matches();

    let (send_channel, receive_channel): (Sender<Value>, Receiver<Value>) = mpsc::channel();
    // let mut threads: Vec<thread::JoinHandle>;

    let connections = setup_database_thread(send_channel.clone());
    let database_thread = connections.0;

    // threads.push(database_thread);
    let send_db_message_channel = connections.1;

    let character = match matches.value_of("new") {
        Some(character) => character,

        _ => return,
    };

    match character {
        "bard" | "Bard" | "BARD" => {
            println!("we gonna create a bard");
            let bard = DbMessage{
                action: String::from("create"),
                verb: String::from("bard"),
                item: Option::None,
            };
            let result = send_db_message_channel.send(serde_json::to_value(bard).unwrap());
            match result {
                Ok(()) => println!("creating bard"),
                Err(error) => {
                    println!("{}", error);
                }
            }
        }
        _ => println!("Not a creatable class."),
    }
}

fn setup_database_thread(
    sender_channel: Sender<Value>,
) -> (std::thread::JoinHandle<()>, Sender<Value>) {
    let connection: (DatabaseConnection, Sender<Value>) =
        db_lib::get_database_connection(sender_channel);

    let database = connection.0;
    let database_send_message_channel = connection.1;

    let database_thread = thread::spawn(move || database.run());

    (database_thread, database_send_message_channel)
}

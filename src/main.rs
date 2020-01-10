#[macro_use]
extern crate clap;
use clap::App;

mod structs;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

// use class::Class;
use rusqlite::{Connection, Error};
// use class::SqlStructure;
use db_lib;
use db_lib::structs::data_connection::DatabaseConnection;

fn main() {
    let yaml = load_yaml!("configuration.yaml");
    let matches = App::from(yaml).get_matches();

    let (send_channel, receive_channel): (Sender<String>, Receiver<String>) = mpsc::channel();
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
            let result = send_db_message_channel.send("create_bard".to_string());
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
    sender_channel: Sender<String>,
) -> (std::thread::JoinHandle<()>, Sender<String>) {
    let connection: (DatabaseConnection, Sender<String>) =
        db_lib::get_database_connection(sender_channel);

    let database = connection.0;
    let database_send_message_channel = connection.1;

    let database_thread = thread::spawn(move || database.run());

    (database_thread, database_send_message_channel)
}

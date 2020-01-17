use crate::structs::class::{Class, CLASSES};
use rusqlite::{Connection, Result};

use serde_json::Value;

use serde::{Deserialize, Serialize};

use std::result::Result as StdResult;
use std::str::FromStr;
use std::sync::mpsc::{self, Receiver, Sender, TryRecvError};
use std::time::Duration;
use std::{error, fmt, thread};

static ACTIONS: [&str; 2] = ["create", "exit"];

#[derive(Debug)]
pub struct DatabaseConnection {
    pub connection: Connection,
    pub output_channel: Sender<Value>,
    pub intake_channel: Receiver<Value>,
}

impl DatabaseConnection {
    pub fn new(output_channel: Sender<Value>) -> (Self, Sender<Value>) {
        let path = "./data/dnd.db";
        let (db_output_channel, db_intake_channel): (Sender<Value>, Receiver<Value>) =
            mpsc::channel();
        let connection = DatabaseConnection {
            connection: connect(&path).unwrap(),
            output_channel,
            intake_channel: db_intake_channel,
        };
        (connection, db_output_channel)
    }

    pub fn run(&self) {
        let check = true;
        while check {
            match self.intake_channel.try_recv() {
                Ok(message) => {
                    // let message: DbMessage = serde_json::from_value(message).unwrap();
                    println!("message: {}", message);
                    // self.parse_action(message);
                }
                Err(error) => match error {
                    TryRecvError::Empty => thread::sleep(Duration::from_secs(1)),
                    TryRecvError::Disconnected => thread::sleep(Duration::from_secs(1)),
                },
            }
        }
    }

    // fn parse_action(&self, message: DbMessage) {
    //     match message.action.as_str() {
    //         "create" => self.create(message),
    //         _ => panic!("Action not implemented"),
    //     }
    // }

    fn create(&self, message: DbMessage) {
        let possible = CLASSES.iter().any(|item| item == &message.noun);
        match possible {
            true => print!("create the class"),
            false => print!("don't create anything"),
        }
    }
}

fn connect(path: &str) -> Result<Connection> {
    match Connection::open(path) {
        Ok(connection) => Ok(connection),
        Err(error) => {
            eprintln!("Could not establish connection: {}", error);
            Err(error)
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbMessage {
    action: String,
    noun: String,
    item: Option<String>,
}

impl DbMessage {
    pub fn new(message: String) -> Self {
        let message_iter = message.split('_').peekable();

        let mut action: String = "action".to_string();
        let mut noun: String = "noun".to_string();
        let mut item: Option<String> = None;

        message_iter.for_each(
            |part| match check_message_part(part.to_lowercase().as_str()) {
                Some("action") => action = part.to_string(),
                Some("noun") => noun = part.to_string(),
                Some("item") => item = Some(part.to_string()),
                None => item = None,
                Some(_) => panic!("Unusable option"),
            },
        );

        DbMessage { action, noun, item }
    }
}

impl fmt::Display for DbMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let item = self.item.clone();
        match item {
            Some(_) => write!(f, "{}_{}_{}", self.action, self.noun, item.unwrap()),
            None => write!(f, "{}_{} ", self.action, self.noun),
        }
    }
}

impl FromStr for DbMessage {
    type Err = ParseError;
    fn from_str(s: &str) -> StdResult<Self, Self::Err> {
        let s = String::from(s);
        let parts: Vec<&str> = s.split("_").collect();
        if parts.len() == 3 {
            Ok(DbMessage {
                action: parts[0].to_string(),
                noun: parts[1].to_string(),
                item: Some(parts[2].to_string()),
            })
        } else if parts.len() == 2 {
            Ok(DbMessage {
                action: parts[0].to_string(),
                noun: parts[1].to_string(),
                item: None,
            })
        } else {
            return Err(ParseError);
        }
    }
}

fn check_message_part(part: &str) -> std::option::Option<&str> {
    let actions = vec!["create", "exit"];
    let nouns = vec!["bard", "application"];
    let items = vec!["item"];

    if ACTIONS.iter().any(|item| item == &part) {
        Some("action")
    } else if nouns.iter().any(|item| item == &part) {
        Some("noun")
    } else if items.iter().any(|item| item == &part) {
        Some("item")
    } else {
        None
    }
}

#[derive(Debug, Clone)]
pub struct ParseError;

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Message length was not 2 or 3")
    }
}

// This is important for other errors to wrap this one.
impl error::Error for ParseError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}

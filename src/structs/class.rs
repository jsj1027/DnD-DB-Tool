use rand::prelude::*;
use rusqlite::{Connection, Error, Statement, NO_PARAMS};
use serde::{Deserialize, Serialize};
use std::cmp::Eq;

#[derive(Debug, Eq, Serialize, Deserialize)]
pub struct Class {
    name: String,
    primary_stat: String,
    secondary_stat: String,
    alternative_stat: String,
    cooperative_stat: String,
}

impl Class {
    pub fn new(database_connection: &Connection, class_name: &str) -> Self {
        let prepared_statement =
            String::from("SELECT name, primaryStat, secondaryStat, cooperativeStat, alternativeStat FROM Class WHERE name='") + &class_name + "'";

        let mut statement: Statement = match database_connection.prepare(&prepared_statement) {
            Ok(statement) => statement,
            Err(error) => panic!(
                "Unable to create a class of name {}. Error:{:#?}",
                &class_name, error
            ),
        };
        let class: Class = match statement.query_row(NO_PARAMS, |row| {
            Ok(Class {
                name: row.get(0)?,
                primary_stat: row.get(1)?,
                secondary_stat: row.get(2)?,
                alternative_stat: row.get(3)?,
                cooperative_stat: row.get(4)?,
            })
        }) {
            Ok(class) => class,
            Err(error) => panic!(
                "Unable to create a class struct data of class name {}. Error: {:#?}",
                &class_name, error
            ),
        };
        class
    }

    pub fn random(database_connection: &Connection) -> Self {
        let class_name: String = super::class::get_random_class_name();
        let prepared_statement =
            String::from("SELECT name, primaryStat, secondaryStat, cooperativeStat, alternativeStat FROM Class WHERE name='") + &class_name + "'";

        let mut statement: Statement = database_connection.prepare(&prepared_statement).unwrap();
        let row: Result<Class, Error> = statement.query_row(NO_PARAMS, |row| {
            Ok(Class {
                name: row.get(0)?,
                primary_stat: row.get(1)?,
                secondary_stat: row.get(2)?,
                alternative_stat: row.get(3)?,
                cooperative_stat: row.get(4)?,
            })
        });
        row.unwrap()
    }
}

impl PartialEq for Class {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

// impl SqlStructure for Class {

// }

fn get_random_class_name() -> String {
    let mut rng_instance = rand::thread_rng();
    let random_class_number = rng_instance.gen_range(0, NUM_OF_CLASSES - 1);
    let class_name = CLASSES[random_class_number];
    String::from(class_name)
}

pub static CLASSES: &'static [&'static str] = &[
    "Barbarian",
    "Bard",
    "Cleric",
    "Druid",
    "Fighter",
    "Monk",
    "Paladin",
    "Ranger",
    "Rogue",
    "Sorcerer",
    "Warlock",
    "Wizard",
];

pub static NUM_OF_CLASSES: usize = CLASSES.len();
#[cfg(test)]
mod tests {
    use super::get_random_class_name;
    use crate::structs::class::{Class, CLASSES};
    use crate::structs::data_connection::DatabaseConnection;
    use rusqlite::Connection;

    fn setup() -> Connection {
        DatabaseConnection::new_test()
    }

    #[test]
    fn test_random() {
        let connection: Connection = setup();

        let class: Class = Class::random(&connection);
        let mut class_name_iter = CLASSES.into_iter();
        assert!(class_name_iter.any(|&class_name| class_name == class.name));
    }

    #[test]
    fn test_get_random_class_name() {
        let connection: Connection = setup();

        let random_class_name: String = get_random_class_name();
        let class: Class = Class::new(&connection, &random_class_name);

        let mut class_name_iter = CLASSES.into_iter();
        assert!(class_name_iter.any(|&class_name| class_name == class.name));
    }

    #[test]
    fn test_new_all_classes() {
        let connection: Connection = setup();

        let class_name_iter = vec![
            "Barbarian",
            "Bard",
            "Cleric",
            "Druid",
            "Fighter",
            "Monk",
            "Paladin",
            "Ranger",
            "Rogue",
            "Sorcerer",
            "Warlock",
            "Wizard",
        ]
        .into_iter();

        let class_name_iter =
            class_name_iter.map(|class_name| Class::new(&connection, &class_name).name);

        let static_class_name_iter = Vec::from(CLASSES).into_iter();
        assert_eq!(class_name_iter.eq(static_class_name_iter), true);
    }

    #[test]
    #[should_panic]
    fn test_bad_class_name() {
        let connection: Connection = setup();
        let class_name: &str = "Wrong";
        let class: Class = Class::new(&connection, &class_name);
    }
}

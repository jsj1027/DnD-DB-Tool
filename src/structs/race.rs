use crate::structs::data_connection::DatabaseConnection;
use crate::structs::status::Ability_Score_Bonus;
use rand::{thread_rng, Rng};
use rusqlite::{Connection, Statement, NO_PARAMS};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize, Serialize)]
pub enum Size {
    Tiny,
    Small,
    Medium,
    Large,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Race {
    pub name: String,
    pub age: u32,
    pub height: f64,
    pub size: Size,
    pub speed: u32,
    pub languages: Vec<String>,
    pub weapon_proficiencies: Option<Vec<String>>,
    pub tool_proficiencies: Option<Vec<String>>,
    pub stat_bonuses: Option<Vec<Ability_Score_Bonus>>,
}

impl Race {
    pub fn new(database_connection: &Connection, race_name: &str) -> Self {
        let prepared_statement = String::from("SELECT * FROM Race WHERE name='") + &race_name + "'";

        let mut statement: Statement = match database_connection.prepare(&prepared_statement) {
            Ok(statement) => statement,
            Err(error) => panic!(
                "Unable to create a race of name {}. Error:{:#?}",
                &race_name, error
            ),
        };
        let race = match statement.query_row(NO_PARAMS, |row| {
            let measurements = Race::get_size_and_height(row.get(1)?, row.get(2)?);
            let height = measurements.0;
            let size = measurements.1;
            let age = Race::get_age(row.get(4)?, row.get(5)?);
            let languages = Race::compose_languages(row.get(6)?);
            let weapon_proficiencies = Race::get_weapon_proficiencies(row.get(9)?);
            let tool_proficiencies = Race::get_tool_proficiencies(row.get(10)?);
            let stat_bonuses = Race::get_stat_bonuses(row.get(11)?);

            Ok(Race {
                name: row.get(0)?,
                size,
                age,
                height,
                speed: row.get(3)?,
                languages: languages,
                weapon_proficiencies,
                tool_proficiencies,
                stat_bonuses,
            })
        }) {
            Ok(race) => race,
            Err(error) => panic!(
                "Unable to create a race struct data of race name {}. Error: {:#?}",
                &race_name, error
            ),
        };
        race
    }

    pub fn random(database_connection: &Connection) -> Self {
        let race_name: String = super::race::get_random_race_name();
        let prepared_statement = String::from("SELECT * FROM Race WHERE name='") + &race_name + "'";

        let mut statement: Statement = match database_connection.prepare(&prepared_statement) {
            Ok(statement) => statement,
            Err(error) => panic!(
                "Unable to create a race of name {}. Error:{:#?}",
                &race_name, error
            ),
        };
        let race = match statement.query_row(NO_PARAMS, |row| {
            let measurements = Race::get_size_and_height(row.get(1)?, row.get(2)?);
            let height = measurements.0;
            let size = measurements.1;
            let age = Race::get_age(row.get(4)?, row.get(5)?);
            let languages = Race::compose_languages(row.get(6)?);
            let weapon_proficiencies = Race::get_weapon_proficiencies(row.get(9)?);
            let tool_proficiencies = Race::get_tool_proficiencies(row.get(10)?);
            let stat_bonuses = Race::get_stat_bonuses(row.get(11)?);

            Ok(Race {
                name: row.get(0)?,
                size,
                age,
                height,
                speed: row.get(3)?,
                languages: languages,
                weapon_proficiencies,
                tool_proficiencies,
                stat_bonuses,
            })
        }) {
            Ok(race) => race,
            Err(error) => panic!(
                "Unable to create a race struct data of race name {}. Error: {:#?}",
                &race_name, error
            ),
        };
        race
    }

    fn get_size_and_height(minimum_size: f64, maximum_size: f64) -> (f64, Size) {
        let mut rng = thread_rng();
        let height: f64 = rng.gen_range(minimum_size, maximum_size);

        let size = match (1.0..1.9).contains(&height) {
            true => Size::Tiny,
            _ => match (2.0..3.9).contains(&height) {
                true => Size::Small,
                _ => match (4.0..7.9).contains(&height) {
                    true => Size::Medium,
                    _ => match (8.0..15.9).contains(&height) {
                        true => Size::Large,
                        _ => panic!("Character size not within possible range"),
                    },
                },
            },
        };
        (height, size)
    }

    fn get_age(minimum_age: u32, maximum_age: u32) -> u32 {
        let mut rng = thread_rng();
        let age: u32 = rng.gen_range(minimum_age, maximum_age);
        age
    }

    fn compose_languages(language: String) -> Vec<String> {
        vec![String::from("Common"), language]
    }

    fn get_weapon_proficiencies(unparsed_weapon_proficiencies: String) -> Option<Vec<String>> {
        let json_value: Value = match serde_json::from_str(&unparsed_weapon_proficiencies) {
            Ok(json_value) => json_value,
            Err(error) => panic!(
                "DB weaponProf section is not in proper json format. Resulting in error: {:#?}",
                error
            ),
        };
        let vectorized_weapon_proficiencies: Option<Vec<String>> =
            match serde_json::from_value(json_value["weapons"].clone()) {
                Ok(vectored_weapon_section) => return Some(vectored_weapon_section),
                Err(error) => return None, // panic!(
                                           // "DB race weaponProf section missing weapons section. Resulting in error: {:#?}",
                                           // error.classify()
                                           // ),
            };
        // Some(vectorized_weapon_proficiencies)
    }

    fn get_tool_proficiencies(unparsed_tool_proficiencies: String) -> Option<Vec<String>> {
        let json_value: Value = match serde_json::from_str(&unparsed_tool_proficiencies) {
            Ok(json_value) => json_value,
            Err(error) => panic!(
                "DB toolProf section is not in proper json format. Resulting in error: {:#?}",
                error
            ),
        };
        let vectorized_tool_proficiencies: Option<Vec<String>> =
            match serde_json::from_value(json_value["tools"].clone()) {
                Ok(vectored_tool_section) => return Some(vectored_tool_section),
                Err(error) => return None, // panic!(
                                           //     "DB race toolProf section missing tools section. Resulting in error: {:#?}",
                                           //     error
                                           // ),
            };
        // Some(vectorized_tool_proficiencies)
    }

    fn get_stat_bonuses(unparsed_stat_bonuses: String) -> Option<Vec<Ability_Score_Bonus>> {
        let json_value: Value = match serde_json::from_str(&unparsed_stat_bonuses) {
            Ok(json_value) => json_value,
            Err(error) => panic!(
                "DB toolProf section is not in proper json format. Resulting in error: {:#?}",
                error
            ),
        };
        let vectorized_stat_bonus_json: Vec<Value> =
            serde_json::from_value(json_value["stat_bonuses"].clone()).unwrap();
        let mut vectorized_ability_score_bonus: Vec<Ability_Score_Bonus> = Vec::new();
        for stat_bonus_value in vectorized_stat_bonus_json {
            &vectorized_ability_score_bonus.push(serde_json::from_value(stat_bonus_value).unwrap());
        }
        Some(vectorized_ability_score_bonus)
    }
}

// impl SqlStructure for Race {

// }

pub static RACES: &'static [&'static str] = &["Dwarf", "Elf", "Human", "Halfling", "Dragonborn"];

pub static NUM_OF_RACES: usize = RACES.len();

fn get_random_race_name() -> String {
    let mut rng_instance = rand::thread_rng();
    let random_race_number = rng_instance.gen_range(0, NUM_OF_RACES - 1);
    let race_name = RACES[random_race_number];
    String::from(race_name)
}

#[cfg(test)]
mod tests {}

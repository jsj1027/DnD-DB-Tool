// pub trait SqlStructure {
//     fn get_table_name() -> String;

//     fn map_to_struct(hash_map: HashMap<String, String>) -> Self;

//     fn new(class_name: &str) -> Self {
//         let data_base = DatabaseConnection::new();
//         let table_name = SqlStructure::get_table_name();
//         let sql_query =
//             String::from("SELECT * FROM ") + &table_name + " WHERE name=" + &class_name;
//         let mut statement = data_base.connection.prepare(&sql_query[..]).unwrap();
//         let column_names = statement.column_names();
//         let mut index_map = HashMap::new();
//         for name in column_names {
//             index_map.insert(name, statement.column_index(&name).unwrap());
//         }
//         let mut statement = data_base.connection.prepare(&sql_query[..]).unwrap();
//         let mut row = statement.query(NO_PARAMS).unwrap();
//         let thing = row.next().unwrap().unwrap();
//         let mut attribute_map = HashMap::new();
//         for (key, value) in index_map.iter() {
//             let some: String = thing.get_unwrap(*value);
//             attribute_map.insert(String::from(*key), some);
//         }
//         println!("{:#?}", attribute_map);
//         SqlStructure::map_to_struct(attribute_map)
//     }

//     fn random_new() {
//         let data_base = DatabaseConnection::new();
//         let table_name = <Class as SqlStructure>::get_table_name();
//         let sql_query =
//             String::from("SELECT * FROM ") + &table_name + " ORDER BY RANDOM() LIMIT 1";
//         let mut statement = data_base.connection.prepare(&sql_query[..]).unwrap();
//         let mut row = statement.query(NO_PARAMS).unwrap();
//         let primary_stat: String = row.next().unwrap().unwrap().get_unwrap(0);
//         println!("primstat {:#?}", primary_stat);
//     }
// }

#[derive(Debug)]
pub struct Class {
    pub name: String,
    pub primary_stat: String,
    pub secondary_stat: String,
    pub alternative_stat: String,
    pub cooperative_stat: String,
}

pub static CLASSES: &'static [&'static str] = &["barbarian","bard","cleric","druid","fighter","monk","paladin","ranger","rogue","sorcerer", "warlock","wizard"];
// impl SqlStructure for Class {

// }

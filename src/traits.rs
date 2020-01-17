pub trait SqlStructure {
    pub fn new() -> Self;

    pub fn new_random() -> Self;

    fn get_struct_name() -> str;

    fn get_table_name() -> str;

    fn add_string_to_query_end(query: &str, string: &str) -> String {
       let mut query = query + string;
       query
    }
        
    fn get_new_attribute(attribute: str) -> String;
}

pub trait Get{
    pub fn get();
}
pub trait PUT{
    pub fn put();
}
pub trait POST {
    pub fn post();
}
pub trait DELETE {
    pub fn delete();
}
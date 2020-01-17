use std::cmp::Eq;

#[derive(Debug, Eq)]
pub struct Class {
    name: String,
    primary_stat: String,
    secondary_stat: String,
    alternative_stat: String,
    cooperative_stat: String,
}

impl Class {
    pub fn new() -> Self {
        Class {
            name: String::from("()"),
            primary_stat: String::from("()"),
            secondary_stat: String::from("()"),
            alternative_stat: String::from("()"),
            cooperative_stat: String::from("()"),
        }
    }
}

impl PartialEq for Class {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}


// impl SqlStructure for Class {

// }

pub static CLASSES: &'static [&'static str] = &[
    "barbarian",
    "bard",
    "cleric",
    "druid",
    "fighter",
    "monk",
    "paladin",
    "ranger",
    "rogue",
    "sorcerer",
    "warlock",
    "wizard",
];

#[cfg(test)]
mod tests {
    use crate::structs::class::Class;
    #[test]
    fn new_class() {
        let class = Class::new();
        assert_eq!(
            class,
            Class {
                name: String::from("()"),
                primary_stat: String::from("()"),
                secondary_stat: String::from("()"),
                alternative_stat: String::from("()"),
                cooperative_stat: String::from("()")
            }
        );
    }
}

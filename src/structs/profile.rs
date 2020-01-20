// use fake::faker::name::raw::{Name, Suffix, Title};
// use fake::locales::EN;
// use fake::Fake;

// use rand::prelude::{thread_rng, Rng, ThreadRng};

// #[derive(Debug)]
// pub struct Profile {
//     pub name: String,
// }

// impl Profile {
//     pub fn new() -> Profile {
//         Profile { name: new_name() }
//     }
// }

// pub fn new_name() -> String {
//     let mut rng: ThreadRng = thread_rng();
//     let chance: bool = rng.gen_ratio(1, 2);
//     let name: String = Name(EN).fake();
//     if chance {
//         let mut title: String = Title(EN).fake();
//         let suffix: String = Suffix(EN).fake();
//         title = match suffix == "PhD" || suffix == "MD" || suffix == "DDS" {
//             true => "Dr.".to_string(),
//             false => title,
//         };
//         title + " " + &name + " " + &suffix.to_string()
//     } else {
//         name
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use spectral::prelude::*;

//     // Test to show that each name has 3 parts.
//     #[test]
//     fn test_new_name() {
//         let name = new_name();
//         let name_vec: Vec<_> = name.split_whitespace().collect();
//         println!("{:#?}", name_vec);
//         assert_that(&name_vec.len()).is_less_than_or_equal_to(4);
//         assert_that(&name_vec.len()).is_greater_than_or_equal_to(2);
//         assert_that(&name_vec.len()).is_not_equal_to(3);
//     }
// }

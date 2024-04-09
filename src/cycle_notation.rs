// use std::collections::HashMap;

// #[derive(PartialEq)]
// pub struct CycleNotation {
//     pub mappings: HashMap<char, char>
// }


// impl CycleNotation {
//      pub fn from_string(s: &str) -> Self {
//         let mut mappings: Vec<Vec<char>> = vec![];
//         let mut normalised_mappings: HashMap<char, char> = HashMap::new();

//         s[1..(s.len() - 2)]
//             .replace(' ', "")
//             .split(")(")
//             .map(|s| s.chars().collect::<Vec<_>>())
//             .for_each(|v| mappings.push(v));

//         for group in mappings {
//             for (idx, c) in group.iter().enumerate() {
//                 if idx == group.len() - 1 {
//                     normalised_mappings.insert(*c, group[0]);
//                 } else {
//                     normalised_mappings.insert(*c, group[idx + 1]);
//                 }
//             }
//         }

//         Self {
//             mappings: normalised_mappings
//         }
//     }
// }


// impl std::fmt::Debug for CycleNotation {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.write_str(&self.to_string())
//     }
// }

// impl ToString for CycleNotation {
//     fn to_string(&self) -> String {
//         self.mappings
//             .iter()
//             .map(|(&lc, &rc)| format!("({lc}{rc})"))
//             .collect::<Vec<_>>()
//             .join(" ")
//     }
// }

pub struct CycleNotation
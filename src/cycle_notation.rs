use std::collections::HashMap;

#[derive(PartialEq)]
pub struct CycleNotation {
    pub mappings: HashMap<char, char>
}


impl CycleNotation {
    pub fn from_string(s: &'static str) -> Self {
        let mut mappings: Vec<Vec<char>> = vec![];
        let mut normalised_mappings: HashMap<char, char> = HashMap::new();

        let mut chars = s.chars();

        chars.next();
        chars.next_back();

        let s = chars.as_str().replace(" ", "");
        let split = s.split(")(");

        for group in split {
            mappings.push(group.chars().collect::<Vec<char>>());
        }

        for group in mappings {
            for (idx, c) in group.iter().enumerate() {
                if idx == group.len() - 1 {
                    normalised_mappings.insert(*c, group[0]);
                } else {
                    normalised_mappings.insert(*c, group[idx + 1]);
                }
            }
        }

        Self {
            mappings: normalised_mappings
        }
    }
}


impl std::fmt::Debug for CycleNotation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl ToString for CycleNotation {
    fn to_string(&self) -> String {
        let mut buf: String = String::new();

        for (c, v) in self.mappings.iter() {
            buf.push('(');
            buf.push(c.to_ascii_uppercase());
            buf.push(v.to_ascii_uppercase());
            buf.push(')');
            buf.push(' ');
        }

        buf.pop();

        buf
    }
}

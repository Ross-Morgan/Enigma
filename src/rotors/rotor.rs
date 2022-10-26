use std::str;

use crate::cycle_notation::CycleNotation;

const ALPHABET: &[u8] = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".as_bytes();


#[derive(Debug, Clone)]
pub struct Rotor {
    pub(in super) mapping: Vec<char>,
    pub(in super) start_char: char,
    pub(in super) offset: usize,
    pub name: Option<&'static str>,
}

impl Rotor {
    pub fn from_cycle_notation(notation: CycleNotation, name: Option<&'static str>) -> Self {
        Self {
            mapping: notation.mappings.clone().into_values().collect(),
            start_char: notation.mappings.clone().into_values().nth(0).unwrap(),
            offset: 0,
            name,
        }
    }

    pub fn from_preset(spec: &'static str, name: Option<&'static str>) -> Self {
        Self::from_cycle_notation(CycleNotation::from_string(spec), name)
    }
}


impl Rotor {
    pub(in crate::rotors) fn rotate(&mut self) {
        println!("Original rotor {:?}: {:?}", self.name.unwrap_or(""), self.mapping);

        let mut to: Vec<char> = self.mapping.clone();
        let mut last: Vec<char> = Vec::with_capacity(1);

        // Reverse order of mapping
        to.reverse();

        // Move last element to new vector
        last.push(to.pop().expect("Rotor character set was empty"));

        // Add rest of vector to last element
        last.append(&mut to);

        // Reverse to get original order
        last.reverse();

        println!("Advanced rotor: {:?}", last);

        self.mapping = last;
    }

    pub fn plug(&mut self, c: char) -> char {
        let _ = ALPHABET
            .to_owned()
            .iter()
            .map(|s| *s as char )
            .collect::<Vec<char>>();

        c
    }
}

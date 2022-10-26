use std::str;

const ALPHABET: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";


#[derive(Debug, Clone)]
pub struct Rotor {
    mapping: Vec<char>,
    pub name: Option<&'static str>,
}

impl Rotor {
    pub fn from(spec: &'static str, name: Option<&'static str>) -> Self {
        Self {
            mapping: spec.chars().collect::<Vec<char>>(),
            name
        }
    }

    pub fn advance(&mut self) {
        println!("Original rotor: {:?}", self.mapping);

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

    pub fn map(&self, letter: char) -> char {
        let position = ALPHABET.chars().position(|c| c == letter).expect("Char not in alphabet");

        ALPHABET.chars().nth(position).expect("Letter not in rotor's charset")
    }
}

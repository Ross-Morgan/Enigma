use std::str;

const ALPHABET: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";


#[derive(Debug, Clone)]
pub struct Rotor {
    mapping: Vec<char>
}

impl Rotor {
    pub fn from(preset_str: &'static str) -> Self {
        Self {
            mapping: preset_str.chars().collect::<Vec<char>>()
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

        // REVIEW ?
        let new_mapping: Vec<char> = last;

        println!("Advanced rotor: {:?}", new_mapping);

        self.mapping = new_mapping;
    }

    pub fn map(&self, letter: char) -> char {
        let position = ALPHABET.chars().position(|c| c == letter).expect("Char not in alphabet");

        ALPHABET.chars().nth(position).expect("Letter not in rotor's charset")
    }
}


pub fn load_rotor_preset<const N: usize>(rotor_preset: [&'static str; N]) -> Vec<Rotor> {
    let mut rotor_vec: Vec<Rotor> = Vec::with_capacity(N);

    for rotor_string in rotor_preset.into_iter() {
        rotor_vec.push(Rotor::from(rotor_string));
    }

    rotor_vec
}

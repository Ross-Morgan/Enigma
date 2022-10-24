use std::str;

const ALPHABET: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";


#[derive(Debug, Clone)]
pub struct Rotor {
    mapping: String
}

impl Rotor {
    pub fn from(preset_str: &'static str) -> Self {
        Self {
            mapping: preset_str.to_string()
        }
    }

    pub fn advance(&mut self) {
        let mut to: Vec<u8> = self.mapping.as_bytes().to_owned();
        let mut last: Vec<u8> = Vec::with_capacity(1);

        to.reverse();

        last.push(to.pop().expect("Rotor character set was empty"));

        last.append(&mut to);
        last.reverse();

        let new_mapping: String = str::from_utf8(last.as_slice()).expect("Could not decode mapping").to_string();

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

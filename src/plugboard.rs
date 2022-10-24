use std::collections::HashMap;


const ALPHABET: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";


fn nth_letter(n: usize) -> char {
    ALPHABET.chars().nth(n).expect("Invalid index")
}


#[derive(Debug)]
pub struct Plugboard {
    mappings: HashMap<char, char>,
}

impl Plugboard {
    pub fn new() -> Self {
        let mappings = HashMap::<char, char>::new();

        for _ in 0..13 {

        }


        Self { }
    }
}


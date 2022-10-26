use std::collections::HashMap;

use crate::cycle_notation::CycleNotation;

#[derive(Clone, Debug)]
pub struct Plugboard {
    pub(in super) mappings: HashMap<char, char>
}


impl Plugboard {
    pub fn new() -> Self {
        Self { mappings: HashMap::new() }
    }

    pub fn from_cycle_notation(notation: CycleNotation) -> Self {
        Self { mappings: notation.mappings }
    }
}

impl Plugboard {
    pub fn plug(&self, c: char) -> char {
        *self.mappings.get(&c).expect("Character not in plugboard mappings")
    }
}

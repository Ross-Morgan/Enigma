use std::collections::HashMap;

use crate::cycle_notation::CycleNotation;

#[derive(Clone, Debug)]
pub struct Reflector {
    mappings: HashMap<char, char>
}


impl Reflector {
    pub fn new() -> Self {
        Self { mappings: HashMap::new() }
    }

    pub fn from_cycle_notation(notation: CycleNotation) -> Self {
        Self { mappings: notation.mappings }
    }

    pub fn from_reflector_preset(preset: &'static str) -> Self {
        Self::from_cycle_notation(CycleNotation::from_string(preset))
    }
}


impl Reflector {
    pub fn plug(&self, c: char) -> char {
        *self.mappings.get(&c).expect("Character not in reflector mappings")
    }
}

use std::collections::HashMap;

use crate::cycle_notation::CycleNotation;

#[derive(Clone, Debug)]
pub struct Plugboard {
    pub(in super) mappings: HashMap<char, char>
}


impl Default for Plugboard {
    fn default() -> Self {
        Self::new()
    }
}

impl Plugboard {
    #[must_use] pub fn new() -> Self {
        Self { mappings: HashMap::new() }
    }

    #[must_use] pub fn from_cycle_notation(notation: CycleNotation) -> Self {
        Self { mappings: notation.mappings }
    }
}

impl Plugboard {
    #[must_use] pub fn plug(&self, c: char) -> char {
        *self.mappings.get(&c).expect("Character not in plugboard mappings")
    }
}

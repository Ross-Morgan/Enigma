use crate::cycle_notation::CycleNotation;

#[derive(Clone, Debug)]
pub struct Reflector{
    mappings: Vec<(char, char)>
}


impl Reflector {
    pub fn new() -> Self {
        Self { mappings: vec![] }
    }

    pub fn from_cycle_notation(notation: CycleNotation) -> Self {
        let mut m = Vec::<(char, char)>::new();

        // Loops through groups
        for v in notation.mappings {

            if v.len() != 2 {
                panic!("");
            }

            m.push((
                *v.iter().nth(0).unwrap(),
                *v.iter().nth(1).unwrap(),
            ))
        }

        Self {
            mappings: m
        }
    }

    pub fn from_reflector_preset(preset: &'static str) -> Self {
        Self::from_cycle_notation(CycleNotation::from_string(preset))
    }
}


impl Reflector {
    pub fn plug(&self, c: char) -> char {
        c
    }
}

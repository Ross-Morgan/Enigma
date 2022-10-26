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
}

/// Create reflector struct from a string
pub fn load_reflector_preset(preset: &'static str) -> Reflector {
    Reflector::from_cycle_notation(CycleNotation::from_string(preset))
}

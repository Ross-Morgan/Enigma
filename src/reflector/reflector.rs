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

        for v in notation.mappings {
            m.push((
                *v.iter().nth(0).expect("Notation subsequence has length < 0"),
                *v.iter().nth(1).expect("Notation subsequence has length < 1"),
            ))
        }

        Self {
            mappings: m
        }
    }
}

pub fn load_reflector_preset(preset: &'static str) -> Reflector {
    let c = CycleNotation::from_string(preset);

    return Reflector::from_cycle_notation(c);
}

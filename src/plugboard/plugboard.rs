#[derive(Clone, Debug)]
pub struct Plugboard {
    pub(in super) mappings: Vec<char>
}


impl Plugboard {
    pub fn new() -> Self {
        Self { mappings: vec![] }
    }
}

impl Plugboard {
    pub fn plug(&self, c: char) -> char {
        c
    }
}
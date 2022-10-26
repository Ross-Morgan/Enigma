#[derive(Copy, Clone, Debug)]
pub struct Plugboard{}


impl Plugboard {
    pub fn new() -> Self {
        Self {}
    }
}

impl Plugboard {
    pub fn plug(&self, c: char) -> char {
        c
    }
}
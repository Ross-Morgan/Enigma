use crate::rotors::Rotor;
use crate::rotors::presets::groups;

#[derive(Clone, Debug)]
pub struct RotorStack<const N: usize> {
    pub(in super) rotors: Vec<Rotor>
}

impl<const N: usize> RotorStack<N> {
    pub fn from_preset(preset: [(&'static str, &'static str); N]) -> Self {
        let mut rotors: Vec<Rotor> = vec![];

        for (name, spec) in preset.into_iter() {
            rotors.push(Rotor::from(spec, Some(name)))
        }

        Self {
            rotors
        }
    }
}


impl RotorStack<3> {
    pub fn new() -> Self {
        Self::from_preset(groups::COMMERCIAL)
    }
}

impl RotorStack<5> {
    pub fn new() -> Self {
        Self::from_preset(groups::ROCKET)
    }
}

impl RotorStack<10> {
    pub fn new() -> Self {
        Self::from_preset(groups::TECHNICAL)
    }
}


impl<const N: usize> RotorStack<N> {
    pub fn plug_forwards(&self, c: char) -> char {
        // TODO Implement
        c
    }

    pub fn plug_backwards(&self, c: char) -> char {
        // TODO Implement
        c
    }
}
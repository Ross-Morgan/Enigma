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
            rotors.push(Rotor::from_preset(spec, Some(name)))
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
    pub fn plug_forwards(&mut self, c: char) -> char {
        // TODO Implement
        let mut plugged = c;

        for rotor in self.rotors.iter_mut() {
            plugged = rotor.plug(plugged);

            rotor.rotate();
        }

        plugged
    }

    pub fn plug_backwards(&self, c: char) -> char {
        // TODO Implement
        c
    }
}

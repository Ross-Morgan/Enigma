use crate::rotors::Rotor;
use crate::rotors::presets::groups;

#[derive(Clone, Debug)]
pub struct RotorStack<const N: usize> {
    pub(in super) rotors: Vec<Rotor>
}

impl<const N: usize> RotorStack<N> {
    #[must_use] pub fn from_preset(preset: [(&'static str, &'static str); N]) -> Self {
        let mut rotors: Vec<Rotor> = vec![];

        for (name, spec) in preset {
            rotors.push(Rotor::from_preset(spec, Some(name)));
        }

        Self {
            rotors
        }
    }
}


impl Default for RotorStack<3> {
    fn default() -> Self {
        Self::new()
    }
}

impl RotorStack<3> {
    #[must_use] pub fn new() -> Self {
        Self::from_preset(groups::COMMERCIAL)
    }
}

impl Default for RotorStack<5> {
    fn default() -> Self {
        Self::new()
    }
}

impl RotorStack<5> {
    #[must_use] pub fn new() -> Self {
        Self::from_preset(groups::ROCKET)
    }
}

impl Default for RotorStack<10> {
    fn default() -> Self {
        Self::new()
    }
}

impl RotorStack<10> {
    #[must_use] pub fn new() -> Self {
        Self::from_preset(groups::TECHNICAL)
    }
}


impl<const N: usize> RotorStack<N> {
    pub fn plug_forwards(&mut self, c: char) -> char {
        // TODO Implement
        let mut plugged = c;

        for rotor in &mut self.rotors {
            plugged = rotor.plug(plugged);

            rotor.rotate();
        }

        plugged
    }

    #[must_use] pub fn plug_backwards(&self, c: char) -> char {
        // TODO Implement
        c
    }
}

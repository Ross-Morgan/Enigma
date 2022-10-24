use crate::plugboard::Plugboard;
use crate::reflector::Reflector;

use crate::rotors::{Rotor, load_rotor_preset};
use crate::rotors::tables::groups;

pub struct EngimaBuilder<const N: usize = 3> {
    _plugboard: Option<Plugboard>,
    _rotors: Option<Vec<Rotor>>,
    _reflector: Option<Reflector>,
}

impl EngimaBuilder {
    pub fn plugboard(&mut self, plugboard: Plugboard) {
        self._plugboard = Some(plugboard);
    }

    pub fn reflector(&mut self, reflector: Reflector) {
        self._reflector = Some(reflector);
    }

    pub fn rotors(&mut self, rotors: Vec<Rotor>) {
        self._rotors = Some(rotors);
    }

    pub fn build(self) -> EnigmaMachine {
        let plugboard = self._plugboard.expect("Tried to build with no plugboard");
        let reflector = self._reflector.expect("Tried to build with no reflector");

        let rotors = self._rotors.expect("Tried to build with no rotors");

        EnigmaMachine { plugboard, rotors, reflector }
    }
}


#[derive(Clone, Debug)]
pub struct EnigmaMachine<const N: usize = 3> {
    plugboard: Plugboard,
    rotors: Vec<Rotor>,
    reflector: Reflector,
}


impl<const N: usize> EnigmaMachine<N> {
    pub fn from_preset(preset: [&'static str; N]) -> Self {
        let plugboard: Plugboard = Plugboard::new();
        let reflector: Reflector = Reflector::new();
        let rotors: Vec<Rotor> = load_rotor_preset::<N>(preset);

        Self { plugboard, rotors, reflector }
    }

    pub fn build_with(self) -> EngimaBuilder {
        EngimaBuilder { _plugboard: None, _rotors: None, _reflector: None }
    }
}

impl EnigmaMachine<3> {
    pub fn new() -> Self {
        Self::from_preset(groups::COMMERCIAL)
    }
}

impl EnigmaMachine<5> {
    pub fn new() -> Self {
        Self::from_preset(groups::ROCKET)
    }
}

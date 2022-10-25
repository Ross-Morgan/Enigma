use crate::plugboard::Plugboard;
use crate::reflector::Reflector;

use crate::rotors::{Rotor, load_rotor_preset};
use crate::rotors::presets::groups;

pub struct EngimaBuilder<const N: usize> {
    _plugboard: Option<Plugboard>,
    _rotors: Option<Vec<Rotor>>,
    _reflector: Option<Reflector>,
}

impl<const N: usize> EngimaBuilder<N> {
    pub fn new() -> Self {
        Self { _plugboard: None, _rotors: None, _reflector: None }
    }

    pub fn plugboard(&mut self, plugboard: Plugboard) -> &mut Self {
        self._plugboard = Some(plugboard);

        self
    }

    pub fn reflector(&mut self, reflector: Reflector) -> &mut Self {
        self._reflector = Some(reflector);

        self
    }

    pub fn rotors(&mut self, rotors: Vec<Rotor>) -> &mut Self {
        self._rotors = Some(rotors);

        self
    }

    pub fn build(&self) -> EnigmaMachine<N> {
        let plugboard = self._plugboard.clone().expect("Tried to build with no plugboard");
        let reflector = self._reflector.clone().expect("Tried to build with no reflector");

        let rotors = self._rotors.clone().expect("Tried to build with no rotors");

        EnigmaMachine::<N>::from_parts(plugboard, rotors, reflector)
    }
}


#[derive(Clone, Debug)]
pub struct EnigmaMachine<const N: usize = 3> {
    plugboard: Plugboard,
    rotors: Vec<Rotor>,
    reflector: Reflector,
}


impl<const N: usize> EnigmaMachine<N> {
    pub fn from_parts(plugboard: Plugboard, rotors: Vec<Rotor>, reflector: Reflector) -> Self {
        Self { plugboard, rotors, reflector }
    }

    pub fn from_preset(preset: [&'static str; N]) -> Self {
        let plugboard: Plugboard = Plugboard::new();
        let reflector: Reflector = Reflector::new();
        let rotors: Vec<Rotor> = load_rotor_preset::<N>(preset);

        Self { plugboard, rotors, reflector }
    }

    pub fn build_with() -> EngimaBuilder<N> {
        EngimaBuilder::new()
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

impl EnigmaMachine<10> {
    pub fn new() -> Self {
        Self::from_preset(groups::TECHNICAL)
    }
}

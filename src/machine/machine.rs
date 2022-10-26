use crate::plugboard::Plugboard;
use crate::reflector::Reflector;
use crate::rotors::RotorStack;


pub struct EngimaBuilder<const N: usize> {
    _plugboard: Option<Plugboard>,
    _rotors: Option<RotorStack<N>>,
    _reflector: Option<Reflector>,
}

impl<const R: usize> EngimaBuilder<R> {
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

    pub fn rotors(&mut self, rotors: RotorStack<R>) -> &mut Self {
        self._rotors = Some(rotors);

        self
    }

    pub fn build(&self) -> EnigmaMachine<R> {
        let plugboard = self._plugboard.clone().expect("Tried to build with no plugboard");
        let reflector = self._reflector.clone().expect("Tried to build with no reflector");

        let rotors = self._rotors.clone().expect("Tried to build with no rotors");

        EnigmaMachine::<R>::from_parts(plugboard, rotors, reflector)
    }
}


#[derive(Clone, Debug)]
pub struct EnigmaMachine<const R: usize = 3> {
    pub(in super) plugboard: Plugboard,
    pub(in super) rotor_stack: RotorStack<R>,
    pub(in super) reflector: Reflector,
}


impl<const N: usize> EnigmaMachine<N> {
    pub fn from_parts(plugboard: Plugboard, rotors: RotorStack<N>, reflector: Reflector) -> Self {
        Self { plugboard, rotor_stack: rotors, reflector }
    }

    pub fn from_preset(preset: [(&'static str, &'static str); N]) -> Self {
        let plugboard: Plugboard = Plugboard::new();
        let reflector: Reflector = Reflector::new();
        let rotors: RotorStack<N> = RotorStack::<N>::from_preset(preset);

        Self { plugboard, rotor_stack: rotors, reflector }
    }

    pub fn build_with() -> EngimaBuilder<N> {
        EngimaBuilder::new()
    }
}

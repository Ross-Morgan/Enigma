use crate::plugboard::Plugboard;
use crate::reflector::Reflector;
use crate::rotors::RotorStack;


pub struct EngimaBuilder<const N: usize> {
    plugboard: Option<Plugboard>,
    rotors: Option<RotorStack<N>>,
    reflector: Option<Reflector>,
}

impl<const R: usize> EngimaBuilder<R> {
    pub fn new() -> Self {
        Self { plugboard: None, rotors: None, reflector: None }
    }

    #[must_use]
    pub fn with_plugboard(mut self, plugboard: Plugboard) -> Self {
        self.plugboard = Some(plugboard);
        self
    }

    #[must_use]
    pub fn with_reflector(mut self, reflector: Reflector) -> Self {
        self.reflector = Some(reflector);
        self
    }

    #[must_use]
    pub fn with_rotors(mut self, rotors: RotorStack<R>) -> Self {
        self.rotors = Some(rotors);
        self
    }

    #[must_use]
    pub fn build(self) -> EnigmaMachine<R> {
        let plugboard = self.plugboard.expect("Tried to build with no plugboard");
        let reflector = self.reflector.expect("Tried to build with no reflector");

        let rotors = self.rotors.expect("Tried to build with no rotors");

        EnigmaMachine::from_parts(plugboard, rotors, reflector)
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

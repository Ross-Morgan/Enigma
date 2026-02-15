use crate::prelude::{BidirectionalPlug, Plugboard, Reflector, RotorStack, UnidirectionalPlug};

pub struct EnigmaMachine<const P: usize, const R: usize, const C: usize>(
    Plugboard<P>,
    RotorStack<R, C>,
    Reflector<C>,
);

pub struct EnigmaMachineBuilder<const P: usize, const R: usize, const C: usize>(
    Option<Plugboard<P>>,
    Option<RotorStack<R, C>>,
    Option<Reflector<C>>,
);

impl<const P: usize, const R: usize, const C: usize> EnigmaMachine<P, R, C> {
    pub const fn build() -> EnigmaMachineBuilder<P, R, C> {
        EnigmaMachineBuilder(None, None, None)
    }
}

impl<const P: usize, const R: usize, const C: usize> EnigmaMachineBuilder<P, R, C> {
    pub const fn with_plugboard(self, plugboard: Plugboard<P>) -> Self {
        Self(Some(plugboard), self.1, self.2)
    }

    pub const fn with_rotor_stack(self, rotor_stack: RotorStack<R, C>) -> Self {
        Self(self.0, Some(rotor_stack), self.2)
    }

    pub fn with_rotor_configuration(self, rotor_order: [usize; R]) -> Self {
        match self.1 {
            None => self,
            Some(rs) => {
                let rotors = rs.rotors();
                let reordered = RotorStack::new(rotor_order.map(|i| rotors[i - 1]));
                Self(self.0, Some(reordered), self.2)
            }
        }
    }

    pub const fn with_reflector(self, reflector: Reflector<C>) -> Self {
        Self(self.0, self.1, Some(reflector))
    }

    pub fn finish(self) -> EnigmaMachine<P, R, C> {
        EnigmaMachine(
            self.0
                .expect("Missing plugboard when trying to build enigma machine"),
            self.1
                .expect("Missing rotor stack when trying to build enigma machine"),
            self.2
                .expect("Missing reflector when trying to build enigma machine"),
        )
    }
}

#[macro_export]
macro_rules! chain {
    {
        $s:expr,
        $f:expr,
        $t:tt
    } => {
        $t($s, $crate::chain!($s, $t))
    };
    ($s:expr, $f:expr) => {
        $f
    }
}

impl<const P: usize, const R: usize, const C: usize> EnigmaMachine<P, R, C> {
    pub fn encrypt_str(&mut self, s: &str) -> String {
        s.chars().map(|c| self.encrypt_char(c)).collect()
    }

    pub fn encrypt_char(&mut self, mut c: char) -> char {
        // c = self.0.plug_forwards(c);
        // c = self.1.plug_forwards(c);
        // c = self.2.plug(c);
        // c = self.1.plug_backwards(c);
        // c = self.0.plug_backwards(c);
        // c

        chain!(
            self,
            Plugboard::plug_forwards,
            RotorStack::plug_forwards,
            Reflector::plug,
            RotorStack::plug_backwards,
            Plugboard::plug_backwards,
        )
    }
}

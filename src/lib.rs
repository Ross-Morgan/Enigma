#![deny(clippy::missing_const_for_fn)]

// pub mod machine;

// pub mod plugboard;
// pub mod reflector;
// pub mod rotors;
// pub mod cycle_notation;

mod alphamap;

pub trait Plug {
    fn plug_forwards(&self, c: char) -> c; 
    fn plug_backwards(&self, c: char) -> c; 
}

pub struct EnigmaMachine<const R: usize, const C: usize>(Plugboard<C>, RotorStack<R>, Reflector<R>);
pub struct Plugboard<const C: usize>(AlphaMap<C>);
pub struct Rotor<const C: usize>(AlphaMap<C>);
pub struct RotorStack<const R: usize, const C: usize>([Rotor<C>; R]);
pub struct Reflector<const C: usize>(AlphaMap<C, true>);

struct AlphaMap<const N: usize>([char; N], [char; N]);

impl<const N: usize> AlphaMap<N> {
    fn new(from: [char; N], to: [char; N]) -> Self {
        Self(from, to)
    }

    fn get_forwards(&self, c: char) -> char {
        self.1[self.0.iter().position(|&&v| c == v).expect("char not in forwards mapping")]
    }
    
    fn get_backwards(&self, c: char) -> char {
        self.0[self.1.iter().position(|&&v| c == v).expect("char not in backwards mapping")]
    }
}

impl<const C: usize> Plug for Plugboard<C> {
    fn plug_forwards(&self, c: char) -> char {
        self.0.get_forwards(c)
    }
    
    fn plug_backwards(&self, c: char) -> char {
        self.0.get_backwards(c)
    }
}

impl<const N: usize, const C: usize> Plug for RotorStack<N, C> {
    fn plug_forwards(&self, c: char) -> char {
        self.0
            .iter()
            .fold(c, |plugged, rotor| rotor.plug_forwards(plugged))
        }
        
        fn plug_backwards(&self, c: char) -> char {
            self.0
                .iter()
                .fold(c, |plugged, rotor| rotor.plug_forwards(plugged))
    }
}

impl<const C: usize> Plug for Rotor<C> {
    fn plug_forwards(&self, c: char) -> char {
        self.0.get_forwards(c)
    }
    
    fn plug_backwards(&self, c: char) -> char {
        self.0.get_backwards(c)
    }
}

impl<const C: usize> Plug for Reflector<C> {
    fn plug_forwards(&self, c: char) -> char {
        self.0.get_forwards(c)
    }
    
    fn plug_backwards(&self, c: char) -> char {
        self.0.get_backwards(c)
    }
}

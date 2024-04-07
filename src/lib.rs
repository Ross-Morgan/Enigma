#![deny(clippy::all)]
#![deny(clippy::pedantic)]

pub mod machine;

pub mod plugboard;
pub mod reflector;
pub mod rotors;
pub mod cycle_notation;
pub mod types;

pub mod prelude {
    use super::{machine, plugboard, reflector, rotors};

    pub use machine::EnigmaMachine;
    pub use machine::encrypt::Encrypt;
    pub use plugboard::Plugboard;

    pub use reflector::Reflector;
    pub use reflector::presets as reflector_presets;

    pub use rotors::{Rotor, RotorStack};
    pub use rotors::presets::groups as rotor_presets;

}

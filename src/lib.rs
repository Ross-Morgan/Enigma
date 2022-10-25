pub mod machine;

pub mod plugboard;
pub mod reflector;
pub mod rotors;
pub mod cycle_notation;

pub mod prelude {
    use super::*;

    pub use machine::EnigmaMachine;
    pub use plugboard::Plugboard;

    pub use reflector::{Reflector, load_reflector_preset};
    pub use reflector::presets as reflector_presets;

    pub use rotors::{Rotor, load_rotor_preset};
    pub use rotors::presets::groups as rotor_presets;

}

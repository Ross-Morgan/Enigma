use enigma::prelude::*;

fn main() {
    let mut machine_1 = EnigmaMachine::build()
        .with_plugboard(Plugboard::new(AlphaMap::new(['A', 'E'], ['F', 'B'])))
        .with_rotor_stack(enigma::rotors::presets::COMMERCIAL)
        .with_rotor_configuration([3, 1, 2])
        .with_reflector(enigma::reflector::presets::B)
        .finish();

    let mut machine_2 = EnigmaMachine::build()
        .with_plugboard(Plugboard::new(AlphaMap::new(['A', 'E'], ['F', 'B'])))
        .with_rotor_stack(enigma::rotors::presets::COMMERCIAL)
        .with_rotor_configuration([3, 2, 1])
        .with_reflector(enigma::reflector::presets::B)
        .finish();

    println!("{}", machine_1.encrypt_str("HELLO WORLD!"));
    println!("{}", machine_2.encrypt_str("HELLO WORLD!"));
}

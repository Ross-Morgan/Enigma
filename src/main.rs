use enigma::prelude::*;

use enigma::cycle_notation;


fn main() {
    let plugboard = Plugboard::new();
    let reflector = Reflector::new();
    let rotors = load_rotor_preset(rotor_presets::TECHNICAL);

    let machine: EnigmaMachine<10> = EnigmaMachine::<10>::build_with()
        .plugboard(plugboard)
        .reflector(reflector)
        .rotors(rotors)
        .build();

    println!("{machine:?}")
}

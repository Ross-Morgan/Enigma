use enigma::prelude::*;


fn main() {
    let plugboard = Plugboard::new();
    let reflector = Reflector::new();
    let rotors = RotorStack::from_preset(rotor_presets::TECHNICAL);

    let mut machine: EnigmaMachine<10> = EnigmaMachine::<10>::build_with()
        .with_plugboard(plugboard)
        .with_reflector(reflector)
        .with_rotors(rotors)
        .build();

    println!("{machine:#?}");

    let s = machine.encrypt("How are you?");

    println!("Encrypted: {s:?}");
}

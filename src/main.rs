use enigma::prelude::*;


fn main() {
    let plugboard = Plugboard::new();
    let reflector = Reflector::new();
    let rotors = RotorStack::from_preset(rotor_presets::TECHNICAL);

    let machine: EnigmaMachine<10> = EnigmaMachine::<10>::build_with()
        .plugboard(plugboard)
        .reflector(reflector)
        .rotors(rotors)
        .build();

    println!("{machine:#?}")
}

use enigma::machine::EnigmaMachine;


fn main() {
    let machine: EnigmaMachine<3> = EnigmaMachine::<3>::new();

    println!("{machine:?}")
}

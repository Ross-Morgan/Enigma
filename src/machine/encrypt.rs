use crate::machine::EnigmaMachine;


pub trait Encrypt {
    fn encrypt(&mut self, message: &str) -> String;
}


impl<const R: usize> Encrypt for EnigmaMachine<R> {
    fn encrypt(&mut self, message: &str) -> String {
        let mut encrypted = String::new();

        // For every letter in message
        for mut c in message.chars() {
            // Message through plugboard
            c = self.plugboard.plug(c);

            // Message through rotor stack
            c = self.rotor_stack.plug_forwards(c);

            // Message through reflector
            c = self.reflector.plug(c);

            // Message through reverse rotor stack
            c = self.rotor_stack.plug_backwards(c);

            // Message through plugboard
            c = self.plugboard.plug(c);

            encrypted.push(c)
        }

        encrypted
    }
}

use crate::rotors::Rotor;

#[derive(Clone, Debug)]
pub struct RotorStack<const N: usize> {
    rotors: Vec<Rotor>
}

impl<const N: usize> RotorStack<N> {
    pub fn from_preset(preset: [(&'static str, &'static str); N]) -> Self {
        let mut rotors: Vec<Rotor> = vec![];

        for (name, spec) in preset.into_iter() {
            rotors.push(Rotor::from(spec, Some(name)))
        }

        Self {
            rotors
        }
    }
}

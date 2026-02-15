#![deny(clippy::missing_const_for_fn)]

pub mod alphamap;

pub mod machine;
pub mod plug;
pub mod plugboard;
pub mod reflector;
pub mod rotors;

fn copy_into_array<A, T>(slice: &[T]) -> A
where
    A: Default2 + AsMut<[T]>,
    T: Copy,
{
    let mut a = A::default();
    <A as AsMut<[T]>>::as_mut(&mut a).copy_from_slice(slice);
    a
}

// fn clone_into_array<A, T>(slice: &[T]) -> A
// where
//     A: Default2 + AsMut<[T]>,
//     T: Clone,
// {
//     let mut a = A::default();
//     <A as AsMut<[T]>>::as_mut(&mut a).clone_from_slice(slice);
//     a
// }

trait Default2 {
    fn default() -> Self;
}

impl<T: Default2, const N: usize> Default2 for [T; N] {
    fn default() -> Self {
        [(); N].map(|_| T::default())
    }
}

impl Default2 for char {
    #[inline]
    fn default() -> Self {
        '\0'
    }
}
pub mod prelude {
    use super::*;

    pub use alphamap::{AlphaMap, AlphaMapBuilder};
    pub use machine::EnigmaMachine;
    pub use plug::{BidirectionalPlug, UnidirectionalPlug};
    pub use plugboard::Plugboard;
    pub use reflector::Reflector;
    pub use rotors::{Rotor, RotorStack};
}

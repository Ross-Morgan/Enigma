use crate::{alphamap::AlphaMap, prelude::UnidirectionalPlug};

pub struct Reflector<const C: usize>(AlphaMap<C>);

impl<const C: usize> UnidirectionalPlug for Reflector<C> {
    fn plug(&self, c: char) -> char {
        self.0.get_forwards(c).unwrap_or(c)
    }
}

impl<const C: usize> Reflector<C> {
    pub const fn new(character_map: AlphaMap<C>) -> Self {
        Self(character_map)
    }
}

pub mod presets {
    use crate::prelude::{AlphaMap, Reflector};

    pub const B: Reflector<26> = Reflector::new(AlphaMap::<26>::new_alphabet([
        'Y', 'R', 'U', 'H', 'Q', 'S', 'L', 'D', 'P', 'X', 'N', 'G', 'O', 'K', 'M', 'I', 'E', 'B',
        'F', 'Z', 'C', 'W', 'V', 'J', 'A', 'T',
    ]));
    pub const C: Reflector<26> = Reflector::new(AlphaMap::<26>::new_alphabet([
        'F', 'V', 'P', 'J', 'I', 'A', 'O', 'Y', 'E', 'D', 'R', 'Z', 'X', 'W', 'G', 'C', 'T', 'K',
        'U', 'Q', 'S', 'B', 'N', 'M', 'H', 'L',
    ]));
}

use crate::{alphamap::AlphaMap, prelude::BidirectionalPlug};

pub struct Plugboard<const C: usize>(AlphaMap<C>);

impl<const C: usize> BidirectionalPlug for Plugboard<C> {
    fn plug_forwards(&mut self, c: char) -> char {
        self.0.get_forwards(c).unwrap_or(c)
    }

    fn plug_backwards(&mut self, c: char) -> char {
        self.0.get_backwards(c).unwrap_or(c)
    }
}

impl<const C: usize> Plugboard<C> {
    pub fn new(map: AlphaMap<C>) -> Self {
        Self(map)
    }
}

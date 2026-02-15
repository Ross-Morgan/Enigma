use crate::{
    prelude::{AlphaMap, BidirectionalPlug},
    Default2,
};

#[derive(Copy, Clone)]
pub struct Rotor<const C: usize> {
    character_map: AlphaMap<C>,
    position: usize,
    notches: [Option<char>; 2],
}

#[derive(Copy, Clone)]
pub struct RotorStack<const R: usize, const C: usize>([Rotor<C>; R]);

impl<const R: usize, const C: usize> BidirectionalPlug for RotorStack<R, C> {
    fn plug_forwards(&mut self, mut c: char) -> char {
        // Plug through all but last rotor to avoid out of range index
        for idx in 0..(R - 1) {
            c = self.0[idx].plug_forwards(c);

            if self.0[idx].rotate() {
                self.0[idx + 1].rotate();
            }
        }

        // Plug last rotor seperately
        self.0[R - 1].plug_forwards(c)
    }

    fn plug_backwards(&mut self, mut c: char) -> char {
        // Plug through all but first rotor to avoid out of range index
        for idx in (1..=(R - 1)).rev() {
            c = self.0[idx].plug_forwards(c);

            if self.0[idx].rotate() {
                self.0[idx - 1].rotate();
            }
        }

        // Plug first rotor seperately
        self.0[0].plug_forwards(c)
    }
}

impl<const C: usize> BidirectionalPlug for Rotor<C> {
    fn plug_forwards(&mut self, c: char) -> char {
        self.character_map.get_forwards(c).unwrap_or(c)
    }

    fn plug_backwards(&mut self, c: char) -> char {
        self.character_map.get_backwards(c).unwrap_or(c)
    }
}

impl<const R: usize, const C: usize> RotorStack<R, C> {
    pub const fn new(rotors: [Rotor<C>; R]) -> Self {
        Self(rotors)
    }

    pub const fn rotors(self) -> [Rotor<C>; R] {
        self.0
    }
}

impl<const C: usize> Rotor<C> {
    #[inline]
    pub fn read_char(&self) -> char {
        self.character_map[self.position]
    }

    /// Rotates rotor by 1 position
    ///
    /// # Return
    ///
    /// Returns whether the next rotor should be rotated
    pub fn rotate(&mut self) -> bool {
        self.position = (self.position + 1) % C - 1;
        self.notches.contains(&Some(self.read_char()))
    }

    pub const fn new(map: AlphaMap<C>) -> Self {
        Self {
            character_map: map,
            position: 0,
            notches: [Some('Q'), None],
        }
    }
}

impl<const C: usize> Default2 for Rotor<C> {
    fn default() -> Self {
        Self {
            character_map: AlphaMap::default(),
            position: 0,
            notches: [None, None],
        }
    }
}

pub mod presets {
    use super::{AlphaMap, Rotor, RotorStack};

    pub const COMMERCIAL: RotorStack<3, 26> = RotorStack::new([
        Rotor::new(AlphaMap::<26>::new_alphabet([
            'D', 'M', 'T', 'W', 'S', 'I', 'L', 'R', 'U', 'Y', 'Q', 'N', 'K', 'F', 'E', 'J', 'C',
            'A', 'Z', 'B', 'P', 'G', 'X', 'O', 'H', 'V',
        ])),
        Rotor::new(AlphaMap::<26>::new_alphabet([
            'H', 'Q', 'Z', 'G', 'P', 'J', 'T', 'M', 'O', 'B', 'L', 'N', 'C', 'I', 'F', 'D', 'Y',
            'A', 'W', 'V', 'E', 'U', 'S', 'R', 'K', 'X',
        ])),
        Rotor::new(AlphaMap::<26>::new_alphabet([
            'U', 'Q', 'N', 'T', 'L', 'S', 'Z', 'F', 'M', 'R', 'E', 'H', 'D', 'P', 'X', 'K', 'I',
            'B', 'V', 'Y', 'G', 'J', 'C', 'W', 'O', 'A',
        ])),
    ]);
}

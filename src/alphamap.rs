use std::ops::Index;

#[derive(Copy, Clone, Debug)]
pub struct AlphaMap<const N: usize>([char; N], [char; N]);

impl<const N: usize> AlphaMap<N> {
    pub const fn new(from: [char; N], to: [char; N]) -> Self {
        Self(from, to)
    }

    pub const fn new_alphabet(to: [char; 26]) -> AlphaMap<26> {
        AlphaMap::new(
            [
                'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P',
                'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
            ],
            to,
        )
    }

    pub const fn build() -> AlphaMapBuilder<N> {
        AlphaMapBuilder::build_with()
    }

    pub fn get_forwards(&self, c: char) -> Option<char> {
        Some(self.1[self.0.iter().position(|&v| c == v)?])
    }

    pub fn get_backwards(&self, c: char) -> Option<char> {
        Some(self.0[self.1.iter().position(|&v| c == v)?])
    }
}

pub struct AlphaMapBuilder<const N: usize>(Vec<char>, Vec<char>);

impl<const N: usize> AlphaMapBuilder<N> {
    pub const fn build_with() -> Self {
        Self(vec![], vec![])
    }

    pub fn map(mut self, from: char, to: char) -> Self {
        self.0.push(from);
        self.1.push(to);
        self
    }

    pub fn map_many(
        mut self,
        from: impl Iterator<Item = char>,
        to: impl Iterator<Item = char>,
    ) -> Self {
        self.0.extend(from);
        self.1.extend(to);
        self
    }

    pub fn finish(self) -> AlphaMap<N> {
        AlphaMap::new(
            crate::copy_into_array::<[char; N], _>(&self.0),
            crate::copy_into_array::<[char; N], _>(&self.1),
        )
    }
}

impl<const N: usize> Default for AlphaMap<N> {
    fn default() -> Self {
        Self([char::from(0x0); N], [char::from(0x0); N])
    }
}

impl<const N: usize> Index<usize> for AlphaMap<N> {
    type Output = char;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

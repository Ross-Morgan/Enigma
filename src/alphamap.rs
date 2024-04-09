pub(crate) struct AlphaMapBuilder(Vec<char>, Vec<char>);

impl<const N: usize, const S: bool> AlphaMapBuilder<N, S> {
    type Output = AlphaMap<N, S>;
    
    const fn build_with() -> Self {
        Self(vec![], vec![])
    }

    fn finish(self) -> Self::Output {
        Self::Output::new(
            copy_into_array::<[char; N>], _>(&self.0),
            copy_into_array::<[char; N>], _>(&self.1),
        )
    }
}

fn copy_into_array<A, T>(slice: &[T]) -> A
where
    A: Default + AsMut<[T]>,
    T: Copy,
{
    let mut a = A::default();
    <A as AsMut<[T]>>::as_mut(&mut a).copy_from_slice(slice);
    a
}
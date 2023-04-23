use std::ops::Index;

pub trait Transpose<T, const N: usize>: Index<usize, Output = T>
where
    T: Copy + Default,
{
    fn transpose<F>(&self, f: F) -> [T; N]
    where
        F: Fn(usize) -> usize,
    {
        let mut result = [T::default(); N];

        (0..N).for_each(|index| result[f(index)] = *self.index(index));
        result
    }
}

impl<T, const N: usize> Transpose<T, N> for [T; N] where T: Copy + Default {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transpose_should_do_nothing_when_index_is_returned_as_is() {
        assert_eq!([1, 2, 3, 4].transpose(|index| index), [1, 2, 3, 4]);
    }
}

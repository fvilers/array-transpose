use crate::{coords_to_index, index_to_coords, Transpose};

pub enum FlipAxis {
    Horizontal,
    Vertical,
}

pub trait Flip<T, const N: usize> {
    fn flip(&self, axis: FlipAxis) -> [T; N];
}

impl<T, const N: usize> Flip<T, N> for [T; N]
where
    T: Copy + Default,
{
    fn flip(&self, axis: FlipAxis) -> [T; N] {
        let len = (N as f64).sqrt() as usize;

        self.transpose(|index| {
            let (x, y) = index_to_coords(index, len);
            match axis {
                FlipAxis::Horizontal => coords_to_index(x, len - 1 - y, len),
                FlipAxis::Vertical => coords_to_index(len - 1 - x, y, len),
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flip_horizontal_should_return_a_flipped_array_of_1_element() {
        let actual = [1];
        let result = actual.flip(FlipAxis::Horizontal);

        assert_eq!([1], result);
    }

    #[test]
    fn flip_vertical_should_return_a_flipped_array_of_1_element() {
        let actual = [1];
        let result = actual.flip(FlipAxis::Vertical);

        assert_eq!([1], result);
    }

    #[test]
    fn flip_horizontal_should_return_a_flipped_array_of_4_elements() {
        let actual = [1, 2, 3, 4];
        let result = actual.flip(FlipAxis::Horizontal);

        assert_eq!([3, 4, 1, 2], result);
    }

    #[test]
    fn flip_vertical_should_return_a_flipped_array_of_4_elements() {
        let actual = [1, 2, 3, 4];
        let result = actual.flip(FlipAxis::Vertical);

        assert_eq!([2, 1, 4, 3], result);
    }

    #[test]
    fn flip_horizontal_should_return_a_flipped_array_of_9_elements() {
        let actual = [1, 2, 3, 4, 5, 6, 7, 8, 9];
        let result = actual.flip(FlipAxis::Horizontal);

        assert_eq!([7, 8, 9, 4, 5, 6, 1, 2, 3], result);
    }

    #[test]
    fn flip_vertical_should_return_a_flipped_array_of_9_elements() {
        let actual = [1, 2, 3, 4, 5, 6, 7, 8, 9];
        let result = actual.flip(FlipAxis::Vertical);

        assert_eq!([3, 2, 1, 6, 5, 4, 9, 8, 7], result);
    }
}

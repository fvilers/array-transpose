use crate::{coords_to_index, index_to_coords, Transpose};

pub enum RotationDirection {
    Left,
    Right,
}

pub trait Rotate<T, const N: usize>
where
    T: Copy + Default,
{
    fn rotate(&self, direction: RotationDirection) -> [T; N];
}

impl<T, const N: usize> Rotate<T, N> for [T; N]
where
    T: Copy + Default,
{
    fn rotate(&self, direction: RotationDirection) -> [T; N] {
        let len = (N as f64).sqrt() as usize;

        self.transpose(|index| {
            let (x, y) = index_to_coords(index, len);
            match direction {
                RotationDirection::Left => coords_to_index(len - 1 - y, x, len),
                RotationDirection::Right => coords_to_index(y, len - 1 - x, len),
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rotate_left_should_return_a_rotated_array_of_1_element() {
        let actual = [1];
        let result = actual.rotate(RotationDirection::Left);

        assert_eq!([1], result);
    }

    #[test]
    fn rotate_right_should_return_a_rotated_array_of_1_element() {
        let actual = [1];
        let result = actual.rotate(RotationDirection::Right);

        assert_eq!([1], result);
    }

    #[test]
    fn rotate_left_should_return_a_rotated_array_of_4_elements() {
        let actual = [1, 2, 3, 4];
        let result = actual.rotate(RotationDirection::Left);

        assert_eq!([2, 4, 1, 3], result);
    }

    #[test]
    fn rotate_right_should_return_a_rotated_array_of_4_elements() {
        let actual = [1, 2, 3, 4];
        let result = actual.rotate(RotationDirection::Right);

        assert_eq!([3, 1, 4, 2], result);
    }

    #[test]
    fn rotate_left_should_return_a_rotated_array_of_9_elements() {
        let actual = [1, 2, 3, 4, 5, 6, 7, 8, 9];
        let result = actual.rotate(RotationDirection::Left);

        assert_eq!([3, 6, 9, 2, 5, 8, 1, 4, 7], result);
    }

    #[test]
    fn rotate_right_should_return_a_rotated_array_of_9_elements() {
        let actual = [1, 2, 3, 4, 5, 6, 7, 8, 9];
        let result = actual.rotate(RotationDirection::Right);

        assert_eq!([7, 4, 1, 8, 5, 2, 9, 6, 3], result);
    }
}

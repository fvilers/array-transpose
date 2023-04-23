use crate::{coords_to_index, index_to_coords, Transpose};

pub enum PushDirection {
    Left,
    Right,
}

pub trait Push<T, const N: usize>
where
    T: Copy + Default,
{
    fn push(&self, direction: PushDirection) -> [T; N];
}

impl<T, const N: usize> Push<T, N> for [T; N]
where
    T: Copy + Default,
{
    fn push(&self, direction: PushDirection) -> [T; N] {
        let len = (N as f64).sqrt() as usize;

        self.transpose(|index| {
            let (x, y) = index_to_coords(index, len);
            match direction {
                PushDirection::Left => coords_to_index(x, (y + len - 1) % len, len),
                PushDirection::Right => coords_to_index(x, (y + len + 1) % len, len),
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_left_should_return_a_pushed_array_of_1_element() {
        let actual = [1];
        let result = actual.push(PushDirection::Left);

        assert_eq!([1], result);
    }

    #[test]
    fn push_right_should_return_a_pushed_array_of_1_element() {
        let actual = [1];
        let result = actual.push(PushDirection::Right);

        assert_eq!([1], result);
    }

    #[test]
    fn push_left_should_return_a_pushed_array_of_4_elements() {
        let actual = [1, 2, 3, 4];
        let result = actual.push(PushDirection::Left);

        assert_eq!([2, 1, 4, 3], result);
    }

    #[test]
    fn push_right_should_return_a_pushed_array_of_4_elements() {
        let actual = [1, 2, 3, 4];
        let result = actual.push(PushDirection::Right);

        assert_eq!([2, 1, 4, 3], result);
    }

    #[test]
    fn push_left_should_return_a_pushed_array_of_9_elements() {
        let actual = [1, 2, 3, 4, 5, 6, 7, 8, 9];
        let result = actual.push(PushDirection::Left);

        assert_eq!([2, 3, 1, 5, 6, 4, 8, 9, 7], result);
    }

    #[test]
    fn push_right_should_return_a_pushed_array_of_9_elements() {
        let actual = [1, 2, 3, 4, 5, 6, 7, 8, 9];
        let result = actual.push(PushDirection::Right);

        assert_eq!([3, 1, 2, 6, 4, 5, 9, 7, 8], result);
    }
}

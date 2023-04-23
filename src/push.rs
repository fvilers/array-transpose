use crate::{coords_to_index, index_to_coords, Transpose};

pub enum PushDirection {
    Down,
    Left,
    Right,
    Up,
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
                PushDirection::Down => coords_to_index((x + len + 1) % len, y, len),
                PushDirection::Left => coords_to_index(x, (y + len - 1) % len, len),
                PushDirection::Right => coords_to_index(x, (y + len + 1) % len, len),
                PushDirection::Up => coords_to_index((x + len - 1) % len, y, len),
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_down_should_return_a_pushed_array_of_1_element() {
        let actual = [1];
        let result = actual.push(PushDirection::Down);

        assert_eq!([1], result);
    }

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
    fn push_up_should_return_a_pushed_array_of_1_element() {
        let actual = [1];
        let result = actual.push(PushDirection::Up);

        assert_eq!([1], result);
    }

    #[test]
    fn push_down_should_return_a_pushed_array_of_4_elements() {
        let actual = [1, 2, 3, 4];
        let result = actual.push(PushDirection::Down);

        assert_eq!([3, 4, 1, 2], result);
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
    fn push_up_should_return_a_pushed_array_of_4_elements() {
        let actual = [1, 2, 3, 4];
        let result = actual.push(PushDirection::Up);

        assert_eq!([3, 4, 1, 2], result);
    }

    #[test]
    fn push_down_should_return_a_pushed_array_of_9_elements() {
        let actual = [1, 2, 3, 4, 5, 6, 7, 8, 9];
        let result = actual.push(PushDirection::Down);

        assert_eq!([7, 8, 9, 1, 2, 3, 4, 5, 6], result);
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

    #[test]
    fn push_up_should_return_a_pushed_array_of_9_elements() {
        let actual = [1, 2, 3, 4, 5, 6, 7, 8, 9];
        let result = actual.push(PushDirection::Up);

        assert_eq!([4, 5, 6, 7, 8, 9, 1, 2, 3], result);
    }
}

use crate::Transpose;

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
        let len = self.len();

        self.transpose(|index| match direction {
            PushDirection::Left => (index + len - 1) % len,
            PushDirection::Right => (index + len + 1) % len,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_should_shift_elements_to_the_left() {
        let array = [1, 2, 3, 4];

        assert_eq!(array.push(PushDirection::Left), [2, 3, 4, 1]);
    }

    #[test]
    fn push_should_shift_elements_to_the_right() {
        let array = [1, 2, 3, 4];

        assert_eq!(array.push(PushDirection::Right), [4, 1, 2, 3]);
    }
}

mod flip;
mod push;
mod rotate;
mod transpose;

pub use flip::{Flip, FlipAxis};
pub use push::{Push, PushDirection};
pub use rotate::{Rotate, RotationDirection};
pub use transpose::Transpose;

fn index_to_coords(index: usize, len: usize) -> (usize, usize) {
    let x = index / len;
    let y = index % len;

    (x, y)
}

fn coords_to_index(x: usize, y: usize, len: usize) -> usize {
    (x * len) + y
}

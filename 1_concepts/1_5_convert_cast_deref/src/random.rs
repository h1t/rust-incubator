use std::{array::TryFromSliceError, ops::Deref};

use rand::Rng;

const SIZE: usize = 3;

#[derive(Debug)]
pub struct Random<T>([T; SIZE]);

impl<T> Random<T> {
    pub fn new(points: [T; SIZE]) -> Self {
        Self(points)
    }

    pub fn get(&self) -> &T {
        let index = rand::thread_rng().gen_range(0..SIZE);
        &self.0[index]
    }
}

impl<T> Deref for Random<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.get()
    }
}

impl<T: Copy> TryFrom<&[T]> for Random<T> {
    type Error = TryFromSliceError;

    fn try_from(value: &[T]) -> Result<Self, Self::Error> {
        value.try_into().map(Self)
    }
}

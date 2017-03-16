extern crate num_traits;

mod tests;

use num_traits::{One, Zero};
use std::ops::{Add, Mul};

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Vector2<T> {
    x: T,
    y: T,
}

impl<T> Vector2<T> {
    pub fn new(x: T, y: T) -> Self {
        Vector2 { x: x, y: y, }
    }
}

impl<T: Zero> Vector2<T> {
    pub fn origin() -> Self {
        Self::new(T::zero(), T::zero())
    }
}

impl<T: Zero + One> Vector2<T> {
    pub fn unit_x() -> Self {
        Self::new(T::one(), T::zero())
    }

    pub fn unit_y() -> Self {
        Self::new(T::zero(), T::one())
    }
}

impl<T, U> Add<Vector2<U>> for Vector2<T>
    where   T: Add<U>,
{
    type Output = Vector2<T::Output>;

    fn add(self, rhs: Vector2<U>) -> Self::Output {
        Vector2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<T, U> Mul<U> for Vector2<T>
    where   T: Mul<U>,
            U: Clone,
{
    type Output = Vector2<T::Output>;

    fn mul(self, rhs: U) -> Self::Output {
        Vector2::new(self.x * rhs.clone(), self.y * rhs)
    }
}

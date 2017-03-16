mod tests;

use std::ops::{Add, Mul};

pub fn clamp<T>(value: T, min: T, max: T) -> T
    where T: PartialOrd,
{
    match () {
        () if value < min => min,
        () if value > max => max,
        _ => value
    }
}


fn sum_product<T, U>(a: T, b: U) -> (<T as Add<U>>::Output, <T as Mul<U>>::Output)
    where   T: Add<U> + Mul<U> + Clone,
            U: Clone,
{
    (a.clone() + b.clone(), a * b)
}

pub trait BoolOptionExt {
    fn into_option<T>(self, value: T) -> Option<T>;
}

impl BoolOptionExt for bool {
    fn into_option<T>(self, value: T) -> Option<T> {
        match self {
            true => Some(value),
            _ => None,
        }
    }
}

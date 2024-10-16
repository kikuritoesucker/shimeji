use crate::cmp_from;

/*
    Implementation of complex numbers.
    z = x + yi
*/
use super::*;

#[derive(Debug, Default, Clone, Copy)]
pub struct Complex<T> {
    x: T,
    y: T,
}

impl<T> Complex<T>
where
    T: Default + Copy + num_traits::Float,
{
    pub fn new() -> Self {
        Self {
            x: T::default(),
            y: T::default(),
        }
    }

    pub fn new_from(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn conjugate(&self) -> Self {
        Self {
            x: self.x,
            y: -self.y,
        }
    }

    pub fn abs(&self) -> T {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn inverse(&self) -> Self {
        self.conjugate() / (self.x * self.x + self.y * self.y)
    }
    
}

impl<T> std::ops::Mul for Complex<T>
where
    T: num_traits::Float + Copy,
{
    type Output = Complex<T>;
    fn mul(self, rhs: Self) -> Self::Output {
        let (a, b) = (self.x, self.y);
        let (c, d) = (rhs.x, rhs.y);
        Self {
            x: a * c - b * d,
            y: a * d + b * c,
        }
    }
}

impl<T> std::ops::Neg for Complex<T>
where
    T: num_traits::Float + Copy,
{
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl<T> std::ops::Div<T> for Complex<T>
where
    T: num_traits::Float + Copy,
{
    type Output = Self;
    fn div(self, rhs: T) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl<T> std::ops::Div for Complex<T>
where
    T: num_traits::Float + Copy + Default,
{
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        let (a, b) = (self.x, self.y);
        let (c, d) = (rhs.x, rhs.y);
        Self::new_from(a * c - b * d, a * d + b * c) / (c * c + d * d)
    }
}
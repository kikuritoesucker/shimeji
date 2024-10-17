/*
    Implementation of complex numbers.
    z = x + yi
*/

use super::*;
use crate::cmp_from;
use quaternion::Quaternion;

#[derive(Debug, Default, Clone, Copy)]
pub struct Complex<T> {
    pub x: T,
    pub y: T,
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

    pub fn normalize(&self) -> Self {
        *self / self.abs()
    }

    pub fn dot(&self, other: &Self) -> T {
        self.x * other.x + self.y * other.y
    }

    pub fn abs(&self) -> T {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn inverse(&self) -> Self {
        self.conjugate() / (self.x * self.x + self.y * self.y)
    }

    pub fn lerp(&self, other: &Self, t: T) -> Self {
        *self * (T::one() - t) + *other * t
    }

    pub fn nlerp(&self, other: &Self, t: T) -> Self {
        self.lerp(other, t).normalize()
    }

    pub fn exp(&self) -> Self {
        let (c, theta) = (self.x, self.y);
        Complex::new_from(theta.cos(), theta.sin()) * c.exp()
    }

    pub fn slerp(&self, other: &Self, t: T) -> Self {
        let theta = self.dot(&other).acos();
        let alpha = ((T::one() - t) * theta).sin() / theta.sin();
        let beta = (t * theta).sin() / theta.sin();
        *self * alpha + *other * beta
    }
}

impl<T> std::ops::Add<T> for Complex<T>
where
    T: num_traits::Float + Copy,
{
    type Output = Complex<T>;
    fn add(self, rhs: T) -> Self::Output {
        let (a, b) = (self.x, self.y);
        Self { x: a + rhs, y: b }
    }
}

impl<T> std::ops::Add for Complex<T>
where
    T: num_traits::Float + Copy,
{
    type Output = Complex<T>;
    fn add(self, rhs: Self) -> Self::Output {
        let (a, b) = (self.x, self.y);
        let (c, d) = (rhs.x, rhs.y);
        Self { x: a + c, y: b + d }
    }
}

impl<T> std::ops::Sub<T> for Complex<T>
where
    T: num_traits::Float + Copy,
{
    type Output = Complex<T>;
    fn sub(self, rhs: T) -> Self::Output {
        let (a, b) = (self.x, self.y);
        Self { x: a - rhs, y: b }
    }
}

impl<T> std::ops::Sub for Complex<T>
where
    T: num_traits::Float + Copy,
{
    type Output = Complex<T>;
    fn sub(self, rhs: Self) -> Self::Output {
        let (a, b) = (self.x, self.y);
        let (c, d) = (rhs.x, rhs.y);
        Self { x: a - c, y: b - d }
    }
}

impl<T> std::ops::Mul<T> for Complex<T>
where
    T: num_traits::Float + Copy,
{
    type Output = Complex<T>;
    fn mul(self, rhs: T) -> Self::Output {
        let (a, b) = (self.x, self.y);
        Self {
            x: a * rhs,
            y: b * rhs,
        }
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

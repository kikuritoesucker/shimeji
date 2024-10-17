use std::num;

use complex::Complex;

use crate::{lerp, vec_from};

use super::*;

/*
    Implementation of quaternion.
    H = a + bi + cj + dk
*/
#[derive(Debug, Clone, Copy)]
pub struct Quaternion<T> {
    pub a: T,
    pub b: T,
    pub c: T,
    pub d: T,
}

#[allow(unused)]
impl<T> Quaternion<T>
where
    T: Default + Copy + num_traits::Float,
{
    pub fn new() -> Self {
        Self {
            a: T::default(),
            b: T::default(),
            c: T::default(),
            d: T::default(),
        }
    }

    pub fn new_from(a: T, b: T, c: T, d: T) -> Self {
        Self { a, b, c, d }
    }

    pub fn conjugate(&self) -> Self {
        Self {
            a: self.a,
            b: -self.b,
            c: -self.c,
            d: -self.d,
        }
    }

    pub fn dot(&self, other: &Self) -> T {
        self.a * other.a + self.b * other.b + self.c * other.c + self.d * other.d
    }

    pub fn abs(&self) -> T {
        (self.conjugate() * self.clone()).a.sqrt()
    }

    pub fn inverse(&self) -> Self {
        self.conjugate() / ((self.conjugate() * self.clone()).a)
    }

    pub fn decompose(&self) -> (T, Vec3<T>) {
        (self.a, Vec3::from([[self.b, self.c, self.d]]))
    }

    pub fn rotate(&self, p: &Self) -> Self {
        p.inverse() * *self * *p
    }

    pub fn normalize(&self) -> Self {
        *self / self.abs()
    }

    pub fn lerp(&self, other: &Self, t: T) -> Self {
        *self * (T::one() - t) + *other * t
    }

    pub fn nlerp(&self, other: &Self, t: T) -> Self {
        self.lerp(other, t).normalize()
    }

    pub fn slerp(&self, other: &Self, t: T) -> Self {
        let theta = self.dot(&other).acos();
        let alpha = ((T::one() - t) * theta).sin() / theta.sin();
        let beta = (t * theta).sin() / theta.sin();
        *self * alpha + *other * beta
    }
}

impl<T> From<[T; 4]> for Quaternion<T>
where
    T: Copy,
{
    fn from(value: [T; 4]) -> Self {
        Self {
            a: value[0],
            b: value[1],
            c: value[2],
            d: value[3],
        }
    }
}

impl<T> From<Complex<T>> for Quaternion<T>
where
    T: Copy + num_traits::Float
{
    fn from(value: Complex<T>) -> Self {
        Self {
            a : value.x,
            b : value.y,
            c : T::zero(),
            d : T::zero()
        }
    }
}

// impl<T> std::ops::Index<usize> for Quaternion<T>
// where
//     T: Default + Copy,
// {
//     type Output = T;
//     fn index(&self, index: usize) -> &Self::Output {
//         match index {
//             0 => &self.a,
//             1 => &self.b,
//             2 => &self.c,
//             3 => &self.d,
//             _ => {
//                 panic!("Quaternion index out of range");
//             }
//         }
//     }
// }

// impl<T> std::ops::IndexMut<usize> for Quaternion<T>
// where
//     T: Default + Copy,
// {
//     fn index_mut(&mut self, index: usize) -> &mut Self::Output {
//         match index {
//             0 => &mut self.a,
//             1 => &mut self.b,
//             2 => &mut self.c,
//             3 => &mut self.d,
//             _ => {
//                 panic!("Quaternion index out of range");
//             }
//         }
//     }
// }

impl<T> std::ops::Add<T> for Quaternion<T>
where
    T: Default + Copy + num_traits::Float,
{
    type Output = Quaternion<T>;
    fn add(self, rhs: T) -> Self::Output {
        Self {
            a: self.a + rhs,
            b: self.b,
            c: self.c,
            d: self.d,
        }
    }
}

impl<T> std::ops::Add for Quaternion<T>
where
    T: Default + Copy + num_traits::Float,
{
    type Output = Quaternion<T>;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            a: self.a + rhs.a,
            b: self.b + rhs.b,
            c: self.c + rhs.c,
            d: self.d + rhs.d,
        }
    }
}

impl<T> std::ops::Sub<T> for Quaternion<T>
where
    T: Default + Copy + num_traits::Float,
{
    type Output = Quaternion<T>;
    fn sub(self, rhs: T) -> Self::Output {
        Self {
            a: self.a - rhs,
            b: self.b,
            c: self.c,
            d: self.d,
        }
    }
}

impl<T> std::ops::Sub for Quaternion<T>
where
    T: Default + Copy + num_traits::Float,
{
    type Output = Quaternion<T>;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            a: self.a - rhs.a,
            b: self.b - rhs.b,
            c: self.c - rhs.c,
            d: self.d - rhs.d,
        }
    }
}

impl<T> std::ops::Mul<T> for Quaternion<T>
where
    T: Default + Copy + num_traits::Float,
{
    type Output = Quaternion<T>;
    fn mul(self, rhs: T) -> Self::Output {
        Self {
            a: self.a * rhs,
            b: self.b * rhs,
            c: self.c * rhs,
            d: self.d * rhs,
        }
    }
}

impl<T> std::ops::Mul<Self> for Quaternion<T>
where
    T: Default + Copy + num_traits::Float,
{
    type Output = Quaternion<T>;
    fn mul(self, rhs: Self) -> Self::Output {
        let (a, b, c, d) = (self.a, self.b, self.c, self.d);
        let (t, x, y, z) = (rhs.a, rhs.b, rhs.c, rhs.d);
        Self {
            a: a * t - b * x - c * y - d * z,
            b: a * x + b * t + c * z - d * y,
            c: a * y - b * z + c * t + d * x,
            d: a * z + b * y - c * x + d * t,
        }
    }
}

impl<T> std::ops::Div<T> for Quaternion<T>
where
    T: Default + Copy + num_traits::Float,
{
    type Output = Quaternion<T>;
    fn div(self, rhs: T) -> Self::Output {
        Self {
            a: self.a / rhs,
            b: self.b / rhs,
            c: self.c / rhs,
            d: self.d / rhs,
        }
    }
}

impl<T> std::ops::Div for Quaternion<T>
where
    T: Default + Copy + num_traits::Float,
{
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        self * rhs.inverse() / (rhs.a * rhs.a + rhs.b * rhs.b + rhs.c * rhs.c + rhs.d * rhs.d)
    }
}

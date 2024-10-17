/*
    Implementation of vectors.
*/

use super::aliases::*;
impl<T, const L: usize> TVec<T, L>
where
    T: Default
        + Copy
        + num_traits::Float
        + std::ops::AddAssign
        + std::ops::DivAssign
        + std::ops::Mul<T, Output = T>
        + std::ops::Div<T, Output = T>
        + std::fmt::Display,
    f32: From<T>,
{
    pub fn dot(&self, other: &Self) -> T {
        let mut result = T::default();
        for i in 0..L {
            result += self[0][i] * other[0][i];
        }
        result
    }

    pub fn sum(&self) -> T {
        let mut sum: T = T::default();
        for &i in &self.data[0] {
            sum += i;
        }
        sum
    }

    pub fn length_squared(&self) -> T {
        let mut length_squared = T::default();
        for &i in &self.data[0] {
            length_squared += i * i
        }
        length_squared
    }

    pub fn length(&self) -> T {
        self.length_squared().sqrt()
    }

    pub fn normalize(&self) -> Self {
        self.clone() / self.length()
    }
    
    pub fn lerp(&self, other : &Self, t : T) -> Self {
        *self * (T::one() - t) + *other * t
    }

    pub fn nlerp(&self, other : &Self, t : T) -> Self {
        self.lerp(other, t).normalize()
    }

    pub fn slerp(&self, other : &Self, t : T) -> Self {
        let theta = self.dot(&other).acos();
        let alpha = ((T::one() - t) * theta).sin() / theta.sin();
        let beta = (t * theta).sin() / theta.sin();
        *self * alpha + *other * beta
    }

    // pub fn normalize(&mut self) {
    //     let length = self.length();
    //     for i in 0..L {
    //         self.data[0][i] /= length;
    //     }
    // }
}

impl<T, const U: usize> Into<[T; U]> for TVec<T, U>
where
    T: Copy,
{
    fn into(self) -> [T; U] {
        self.data[0]
    }
}

// Vec3 Methods

impl<T> Vec3<T>
where
    T: Default + Copy + std::ops::Mul<T, Output = T> + std::ops::Sub<T, Output = T>,
{
    pub fn cross(&self, other: &Self) -> Self {
        let (x1, y1, z1) = (self.x(), self.y(), self.z());
        let (x2, y2, z2) = (other.x(), other.y(), other.z());
        Vec3::from([[y1 * z2 - z1 * y2, z1 * x2 - x1 * z2, x1 * y2 - y1 * x2]])
    }
}

// Vec2 Methods

impl<T> Vec2<T>
where
    T: Clone + Copy,
{
    pub fn x(&self) -> T {
        self[0][0]
    }
    pub fn y(&self) -> T {
        self[0][1]
    }

    pub fn x_mut(&mut self) -> &mut T {
        &mut self[0][0]
    }
    pub fn y_mut(&mut self) -> &mut T {
        &mut self[0][1]
    }
}

impl<T> Vec3<T>
where
    T: Clone + Copy,
{
    pub fn x(&self) -> T {
        self[0][0]
    }
    pub fn y(&self) -> T {
        self[0][1]
    }
    pub fn z(&self) -> T {
        self[0][2]
    }

    pub fn x_mut(&mut self) -> &mut T {
        &mut self[0][0]
    }
    pub fn y_mut(&mut self) -> &mut T {
        &mut self[0][1]
    }
    pub fn z_mut(&mut self) -> &mut T {
        &mut self[0][2]
    }
}

impl<T> Vec4<T>
where
    T: Clone + Copy,
{
    pub fn x(&self) -> T {
        self[0][0]
    }
    pub fn y(&self) -> T {
        self[0][1]
    }
    pub fn z(&self) -> T {
        self[0][2]
    }
    pub fn w(&self) -> T {
        self[0][3]
    }

    pub fn x_mut(&mut self) -> &mut T {
        &mut self[0][0]
    }
    pub fn y_mut(&mut self) -> &mut T {
        &mut self[0][1]
    }
    pub fn z_mut(&mut self) -> &mut T {
        &mut self[0][2]
    }
    pub fn w_mut(&mut self) -> &mut T {
        &mut self[0][3]
    }
}

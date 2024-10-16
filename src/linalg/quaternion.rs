/*
    Implementation of quaternion.
    H = a + bi + cj + dk
*/
#[derive(Debug, Clone, Copy)]
pub struct Quaternion<T> {
    a: T,
    b: T,
    c: T,
    d: T,
}

#[allow(unused)]
impl<T> Quaternion<T>
where
    T: Default,
{
    pub fn new() -> Self {
        Self {
            a: T::default(),
            b: T::default(),
            c: T::default(),
            d: T::default(),
        }
    }

    pub fn new_from(a: T, b : T, c: T, d: T) -> Self {
        Self {
            a,
            b,
            c,
            d,
        }
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

impl<T> std::ops::Add for Quaternion<T>
where
    T: Default + Copy + std::ops::Add<T, Output = T>,
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

impl<T> std::ops::Mul<T> for Quaternion<T>
where
    T: Default + Copy + std::ops::Mul<T, Output = T>,
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
    T: Default + Copy + std::ops::Mul<T, Output = T> + std::ops::Add<T, Output = T> + std::ops::Sub<T, Output = T>
{
    type Output = Quaternion<T>;
    fn mul(self, rhs: Self) -> Self::Output {
        let (a1, b1, c1, d1) = (self.a, self.b, self.c, self.d);
        let (a2, b2, c2, d2) = (rhs.a, rhs.b, rhs.c, rhs.d);
        Self {
            a: a1 * a2 - b1 * b2 - c1 * c2 - d1 * d2,
            b: a1 * b2 + b1 * a2 + c1 * d2 - d1 * c2,
            c: a1 * c2 - b1 * d2 + c1 * a1 + d1 * b2,
            d: a1 * d2 + b1 * c2 - c1 * b2 + d1 * a2 
        }
    }
}

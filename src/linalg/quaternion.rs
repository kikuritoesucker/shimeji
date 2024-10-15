/*
    Implementation of quaternion.
*/
#[derive(Debug, Clone, Copy)]
struct Quaternion<T> {
    x: T,
    y: T,
    z: T,
    w: T,
}

#[allow(unused)]
impl<T> Quaternion<T>
where
    T: Default,
{
    fn new() -> Self {
        Self {
            x: T::default(),
            y: T::default(),
            z: T::default(),
            w: T::default(),
        }
    }

    fn new_from(x: T, y: T, z: T, w: T) -> Self {
        Self {
            x: x,
            y: y,
            z: z,
            w: w,
        }
    }
}

impl<T> From<[T; 4]> for Quaternion<T>
where
    T: Copy,
{
    fn from(value: [T; 4]) -> Self {
        Self {
            x: value[0],
            y: value[1],
            z: value[2],
            w: value[3],
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
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}


use super::aliases::*;

impl<T, const L: usize> TVec<T, L>
where
    T: Default + Clone + Copy + std::ops::AddAssign + std::ops::Mul<T, Output = T>,
{
    pub fn dot(&self, other: &Self) -> T {
        let mut result = T::default();
        for i in 0..L {
            result += self[0][i] * other[0][i];
        }
        result
    }

    pub fn normalized(&self) -> Self {
        
    }
}


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

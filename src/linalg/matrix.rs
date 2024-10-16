/*
    Implementation of typed matrix.
*/

/// #### Typed Matrix
/// TMat<T, R, C> creates an RxC matrix of type T.\
/// TMat stores elements in column-major order.\
/// TMat stores elements in column-major order.
/// This means that elements are stored column by column.\
/// For example, a 4x4 matrix:\
///     m00 m01 m02 m03\
///     m10 m11 m12 m13\
///     m20 m21 m22 m23\
///     m30 m31 m32 m33\
/// will be stored in memory as:\
/// m00, m10, m20, m30, m01, m11, m21, m31, m02, m12, m22, m32, m03, m13, m23, m33.
///
/// Example:
/// ```no_run
/// let myMat : TMat<_, 4, 4> = TMat::from(0.0);
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TMat<T, const R: usize, const C: usize> {
    pub data: [[T; R]; C],
}

impl<T, const R: usize, const C: usize> Default for TMat<T, R, C>
where
    T: Default + Copy,
{
    fn default() -> Self {
        Self {
            data: [[T::default(); R]; C],
        }
    }
}

impl<T, const R: usize, const C: usize> From<T> for TMat<T, R, C>
where
    T: Copy,
{
    fn from(value: T) -> Self {
        Self {
            data: [[value; R]; C],
        }
    }
}

impl<T, const R: usize, const C: usize> From<[[T; R]; C]> for TMat<T, R, C>
where
    T: Copy,
{
    fn from(value: [[T; R]; C]) -> Self {
        Self { data: value }
    }
}

impl<T, const R: usize, const C: usize> Into<[[T; R]; C]> for TMat<T, R, C>
where
    T: Copy,
{
    fn into(self) -> [[T; R]; C] {
        self.data
    }
}

impl<T, const U: usize, const R: usize, const C: usize> TryFrom<[T; U]> for TMat<T, R, C>
where
    T: Default + Copy,
{
    type Error = &'static str;
    fn try_from(value: [T; U]) -> Result<Self, Self::Error> {
        if U == R * C {
            let mut result: Self = Self::new();
            let mut index = 0;
            for i in 0..C {
                for j in 0..R {
                    result[i][j] = value[index];
                    index += 1;
                }
            }
            Ok(result)
        } else {
            Err("Vector length does not match the expected size.")
        }
    }
}

impl<T, const R: usize, const C: usize> TMat<T, R, C>
where
    T: Default + Copy,
{
    pub fn new() -> Self {
        Self::default()
    }

    pub fn transpose(&self) -> TMat<T, C, R> {
        let mut transposed: TMat<T, C, R> = TMat::new();
        for i in 0..C {
            for j in 0..R {
                transposed[j][i] = self[i][j];
            }
        }
        transposed
    }
}

// Indexing

impl<T, const R: usize, const C: usize> std::ops::Index<usize> for TMat<T, R, C> {
    type Output = [T; R];
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T, const R: usize, const C: usize> std::ops::IndexMut<usize> for TMat<T, R, C> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

// Add

impl<T, const R: usize, const C: usize> std::ops::Add<Self> for TMat<T, R, C>
where
    T: std::ops::Add<Output = T> + Copy,
    Self: Default,
{
    type Output = TMat<T, R, C>;

    fn add(self, rhs: Self) -> Self::Output {
        let mut result: TMat<T, R, C> = Self::default();
        for i in 0..C {
            for j in 0..R {
                result.data[i][j] = self.data[i][j] + rhs.data[i][j];
            }
        }
        result
    }
}

impl<T, const R: usize, const C: usize> std::ops::Add<T> for TMat<T, R, C>
where
    T: std::ops::Add<Output = T> + Copy,
    Self: Default,
{
    type Output = TMat<T, R, C>;

    fn add(self, rhs: T) -> Self::Output {
        let mut result: TMat<T, R, C> = Self::default();
        for i in 0..C {
            for j in 0..R {
                result.data[i][j] = self.data[i][j] + rhs;
            }
        }
        result
    }
}

// Sub

impl<T, const R: usize, const C: usize> std::ops::Sub<Self> for TMat<T, R, C>
where
    T: std::ops::Sub<Output = T> + Copy,
    Self: Default,
{
    type Output = TMat<T, R, C>;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut result: TMat<T, R, C> = Self::default();
        for i in 0..C {
            for j in 0..R {
                result.data[i][j] = self.data[i][j] - rhs.data[i][j];
            }
        }
        result
    }
}

impl<T, const R: usize, const C: usize> std::ops::Sub<T> for TMat<T, R, C>
where
    T: std::ops::Sub<Output = T> + Copy,
    Self: Default,
{
    type Output = TMat<T, R, C>;

    fn sub(self, rhs: T) -> Self::Output {
        let mut result: TMat<T, R, C> = Self::default();
        for i in 0..C {
            for j in 0..R {
                result.data[i][j] = self.data[i][j] - rhs;
            }
        }
        result
    }
}

// impl<T, const R: usize, const C: usize> std::ops::Neg for TMat<T, R, C>
// where
//     T: std::ops::Sub<Output = T> + Copy,
//     Self: Default,
// {
//     type Output = Self;
//     fn neg(self) -> Self::Output {
//         let result = self;
//         for i in
//     }
// }

// Mul

impl<T, const R: usize, const C: usize> std::ops::Mul<Self> for TMat<T, R, C>
where
    T: std::ops::Mul<Output = T> + Copy,
    Self: Default,
{
    type Output = TMat<T, R, C>;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut result: TMat<T, R, C> = Self::default();
        for i in 0..C {
            for j in 0..R {
                result.data[i][j] = self.data[i][j] * rhs.data[i][j];
            }
        }
        result
    }
}

impl<T, const R: usize, const C: usize> std::ops::Mul<T> for TMat<T, R, C>
where
    T: std::ops::Mul<Output = T> + Copy,
    Self: Default,
{
    type Output = TMat<T, R, C>;

    fn mul(self, rhs: T) -> Self::Output {
        let mut result: TMat<T, R, C> = Self::default();
        for i in 0..C {
            for j in 0..R {
                result.data[i][j] = self.data[i][j] * rhs;
            }
        }
        result
    }
}

impl<T, const A: usize, const B: usize> TMat<T, A, B>
where
    T: std::ops::Mul<T, Output = T> + Default + Copy + std::ops::AddAssign,
{
    pub fn matmul<const C: usize>(lhs: &Self, rhs: &TMat<T, B, C>) -> TMat<T, A, C> {
        let mut product: TMat<T, A, C> = TMat::default();
        for j in 0..C {
            for k in 0..B {
                for i in 0..A {
                    product[j][i] += lhs[k][i] * rhs[j][k];
                }
            }
        }
        product
    }

    pub fn mm<const C: usize>(&self, rhs: &TMat<T, B, C>) -> TMat<T, A, C> {
        Self::matmul(self, rhs)
    }
}

// Div

impl<T, const R: usize, const C: usize> std::ops::Div<T> for TMat<T, R, C>
where
    T: std::ops::Div<Output = T> + Copy,
    Self: Default,
{
    type Output = TMat<T, R, C>;

    fn div(self, rhs: T) -> Self::Output {
        let mut result: TMat<T, R, C> = Self::default();
        for i in 0..C {
            for j in 0..R {
                result.data[i][j] = self.data[i][j] / rhs;
            }
        }
        result
    }
}
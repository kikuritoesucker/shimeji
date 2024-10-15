/*!Vectors are stored in one row for the good of cache locality and performance.
 *
 */
pub use super::matrix::TMat;

pub type TVec<T, const U: usize> = TMat<T, U, 1>;

pub type Vec2<T> = TVec<T, 2>;
pub type Vec3<T> = TVec<T, 3>;
pub type Vec4<T> = TVec<T, 4>;

pub type Vec2f = Vec2<f32>;
pub type Vec3f = Vec3<f32>;
pub type Vec4f = Vec4<f32>;

pub type Vec2i = Vec2<i32>;
pub type Vec3i = Vec3<i32>;
pub type Vec4i = Vec4<i32>;

pub type Vec2u = Vec2<u32>;
pub type Vec3u = Vec3<u32>;
pub type Vec4u = Vec4<u32>;

pub type Mat2<T> = TMat<T, 2, 2>;
pub type Mat3<T> = TMat<T, 3, 3>;
pub type Mat4<T> = TMat<T, 4, 4>;

pub type Mat2f = Mat2<f32>;
pub type Mat3f = Mat3<f32>;
pub type Mat4f = Mat4<f32>;

pub type Mat2i = Mat2<i32>;
pub type Mat3i = Mat3<i32>;
pub type Mat4i = Mat4<i32>;

pub type Mat2u = Mat2<u32>;
pub type Mat3u = Mat3<u32>;
pub type Mat4u = Mat4<u32>;

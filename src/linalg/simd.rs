use super::*;
use std::arch::x86_64::{
    _mm256_add_ps, _mm256_load_ps, _mm256_mul_ps, _mm256_store_ps, _mm_add_ps, _mm_hadd_ps, _mm_load_ps, _mm_mul_ps, _mm_store_ps
};

#[repr(align(16))]
struct AlignedF32([f32; 4]);

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse"))]
impl Mat4f {
    pub fn mm_simd(&self, other: &Self) -> Self {
        let a = &self.transpose().data;
        let b = &other.data;
        let mut f = AlignedF32([0_f32; 4]);
        let mut c: Mat4f = Mat4f::default();
        unsafe {
            for i in 0..4 {
                for j in 0..4 {
                    let m1 = _mm_mul_ps(_mm_load_ps(b[i].as_ptr()), _mm_load_ps(a[j].as_ptr()));
                    let m2 = _mm_hadd_ps(m1, m1);
                    let m3 = _mm_hadd_ps(m2, m2);
                    _mm_store_ps(f.0.as_mut_ptr(), m3);
                    c[i][j] = f.0[0];
                }
            }
        }
        c
    }
}

impl Vec4f {
    pub fn add_simd(&self, other : &Self) -> Self {
        let a = &self.data;
        let b = &other.data;
        let mut f = AlignedF32([0_f32; 4]);
        unsafe{
            let m1 = _mm_add_ps(_mm_load_ps(a[0].as_ptr()), _mm_load_ps(b[0].as_ptr()));
            _mm_store_ps(f.0.as_mut_ptr(), m1);
        }
        Self::from([f.0])
    }

    pub fn dot_simd(&self, other : &Self) -> f32 {
        let a = &self.data;
        let b = &other.data;
        let mut f = AlignedF32([0_f32; 4]);
        unsafe{
            let m1 = _mm_mul_ps(_mm_load_ps(a[0].as_ptr()), _mm_load_ps(b[0].as_ptr()));
            let m2 = _mm_hadd_ps(m1, m1);
            let m3 = _mm_hadd_ps(m2, m2);
            _mm_store_ps(f.0.as_mut_ptr(), m3);
        }
        f.0[0]
    }
}

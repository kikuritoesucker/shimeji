use crate::linalg::*;

pub struct Transform {
    pub model : Mat4f
}

impl Transform {
    pub fn new() -> Self {
        Self {
            model : Mat4f::identity(),
        }
    }

    pub fn transform(&mut self, t : Mat4f) {
        self.model *= t;
    }
}
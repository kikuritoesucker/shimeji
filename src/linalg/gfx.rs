#![allow(unused)]

use super::*;

impl Mat4f {
    pub fn identity() -> Mat4f {
        Mat4f::from([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }
}

// Rotation
impl Mat4f {
    pub fn rotation_x(rad: f32) -> Mat4f {
        let cos_theta = rad.cos();
        let sin_theta = rad.sin();

        Mat4f::from([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, cos_theta, sin_theta, 0.0],
            [0.0, -sin_theta, cos_theta, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn rotation_y(rad: f32) -> Mat4f {
        let cos_theta = rad.cos();
        let sin_theta = rad.sin();

        Mat4f::from([
            [cos_theta, 0.0, -sin_theta, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [sin_theta, 0.0, cos_theta, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn rotation_z(rad: f32) -> Mat4f {
        let cos_theta = rad.cos();
        let sin_theta = rad.sin();

        Mat4f::from([
            [cos_theta, sin_theta, 0.0, 0.0],
            [-sin_theta, cos_theta, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn rotation_xyz(rotation: Vec3f) -> Mat4f {
        Self::rotation_x(rotation.x())
            * Self::rotation_y(rotation.y())
            * Self::rotation_z(rotation.z())
    }

    pub fn rotation_xzy(rotation: Vec3f) -> Mat4f {
        Self::rotation_x(rotation.x())
            * Self::rotation_z(rotation.z())
            * Self::rotation_y(rotation.y())
    }

    pub fn rotation_yxz(rotation: Vec3f) -> Mat4f {
        Self::rotation_y(rotation.y())
            * Self::rotation_x(rotation.x())
            * Self::rotation_z(rotation.z())
    }

    pub fn rotation_yzx(rotation: Vec3f) -> Mat4f {
        Self::rotation_y(rotation.y())
            * Self::rotation_z(rotation.z())
            * Self::rotation_x(rotation.x())
    }

    pub fn rotation_zyx(rotation: Vec3f) -> Mat4f {
        Self::rotation_z(rotation.z())
            * Self::rotation_y(rotation.y())
            * Self::rotation_x(rotation.x())
    }

    pub fn rotation_zxy(rotation: Vec3f) -> Mat4f {
        Self::rotation_z(rotation.z())
            * Self::rotation_x(rotation.x())
            * Self::rotation_y(rotation.y())
    }
}

// Translation
impl Mat4f {
    pub fn translation(transform: Vec3f) -> Mat4f {
        let (x, y, z) = (transform[0][0], transform[0][1], transform[0][2]);
        Mat4f::from([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [x, y, z, 1.0],
        ])
    }

    pub fn translation_xyz(x: f32, y: f32, z: f32) -> Mat4f {
        Mat4f::from([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [x, y, z, 1.0],
        ])
    }
}

// Scale
impl Mat4f {
    pub fn scale(transform: Vec3f) -> Mat4f {
        let (x, y, z) = (transform[0][0], transform[0][1], transform[0][2]);
        Mat4f::from([
            [x, 0.0, 0.0, 0.0],
            [0.0, y, 0.0, 0.0],
            [0.0, 0.0, z, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }
}

impl Mat4f {
    pub fn scale_xyz(x: f32, y: f32, z: f32) -> Mat4f {
        Mat4f::from([
            [x, 0.0, 0.0, 0.0],
            [0.0, y, 0.0, 0.0],
            [0.0, 0.0, z, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }
}

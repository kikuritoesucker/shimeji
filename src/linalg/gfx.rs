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
            .mm(&Self::rotation_y(rotation.y()))
            .mm(&Self::rotation_z(rotation.z()))
    }

    pub fn rotation_xzy(rotation: Vec3f) -> Mat4f {
        Self::rotation_x(rotation.x())
            .mm(&Self::rotation_z(rotation.z()))
            .mm(&Self::rotation_y(rotation.y()))
    }

    pub fn rotation_yxz(rotation: Vec3f) -> Mat4f {
        Self::rotation_y(rotation.y())
            .mm(&Self::rotation_x(rotation.x()))
            .mm(&Self::rotation_z(rotation.z()))
    }

    pub fn rotation_yzx(rotation: Vec3f) -> Mat4f {
        Self::rotation_y(rotation.y())
            .mm(&Self::rotation_z(rotation.z()))
            .mm(&Self::rotation_x(rotation.x()))
    }

    pub fn rotation_zyx(rotation: Vec3f) -> Mat4f {
        Self::rotation_z(rotation.z())
            .mm(&Self::rotation_y(rotation.y()))
            .mm(&Self::rotation_x(rotation.x()))
    }

    pub fn rotation_zxy(rotation: Vec3f) -> Mat4f {
        Self::rotation_z(rotation.z())
            .mm(&Self::rotation_x(rotation.x()))
            .mm(&Self::rotation_y(rotation.y()))
    }

    pub fn rotation_axis(axis: Vec3f, theta: f32) -> Mat4f {
        let (rx, ry, rz) = (axis.data[0][0], axis.data[0][1], axis.data[0][2]);
        let c1 = theta.cos();
        let c2 = 1.0 - c1;
        let s1 = theta.sin();
        let s2 = 1.0 - s1;

        Self::from([
            [
                c1 + rx * rx * c2,
                ry * rx * c2 + rz * s1,
                rz * rx * c2 - ry * s1,
                0.0,
            ],
            [
                rx * ry * c2 - rz * s1,
                c1 + ry * ry * c2,
                rz * ry * c2 + rx * s1,
                0.0,
            ],
            [
                rx * rz * c2 + ry * s1,
                ry * rz * c2 - rx * s1,
                c1 + rz * rz * c2,
                0.0,
            ],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn rotation_qua(rotation: Qua) -> Mat4f {
        let x = rotation.b;
        let y = rotation.c;
        let z = rotation.d;
        let w = rotation.a;

        let xx = x * x;
        let yy = y * y;
        let zz = z * z;
        let xy = x * y;
        let xz = x * z;
        let yz = y * z;
        let wx = w * x;
        let wy = w * y;
        let wz = w * z;

        Mat4f::from([
            [1.0 - 2.0 * (yy + zz), 2.0 * (xy - wz), 2.0 * (xz + wy), 0.0],
            [2.0 * (xy + wz), 1.0 - 2.0 * (xx + zz), 2.0 * (yz - wx), 0.0],
            [2.0 * (xz - wy), 2.0 * (yz + wx), 1.0 - 2.0 * (xx + yy), 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
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

    pub fn scale_xyz(x: f32, y: f32, z: f32) -> Mat4f {
        Mat4f::from([
            [x, 0.0, 0.0, 0.0],
            [0.0, y, 0.0, 0.0],
            [0.0, 0.0, z, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }
}

// Projection

impl Mat4f {
    pub fn ortho(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Mat4f {
        let rl = 1.0 / (right - left);
        let tb = 1.0 / (top - bottom);
        let fnf = 1.0 / (far - near);

        Mat4f::from([
            [2.0 * rl, 0.0, 0.0, 0.0],
            [0.0, 2.0 * tb, 0.0, 0.0],
            [0.0, 0.0, -2.0 * fnf, 0.0],
            [-(right + left) * rl, -(top + bottom) * tb, -(far + near) * fnf, 1.0],
        ])
    }

    pub fn perspective(fov: f32, aspect: f32, near: f32, far: f32) -> Mat4f {
        let f = 1.0 / (fov / 2.0).tan();
        let nf = 1.0 / (near - far);

        Mat4f::from([
            [f / aspect, 0.0, 0.0, 0.0],
            [0.0, f, 0.0, 0.0],
            [0.0, 0.0, (far + near) * nf, -1.0],
            [0.0, 0.0, (2.0 * far * near) * nf, 0.0],
        ])
    }
}


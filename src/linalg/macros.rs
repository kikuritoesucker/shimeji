#[macro_export]
macro_rules! mat4_mm {
    ($a:expr, $b:expr) => {
        crate::linalg::Mat4f::from([
            [
                $a[0][0] * $b[0][0]
                    + $a[1][0] * $b[0][1]
                    + $a[2][0] * $b[0][2]
                    + $a[3][0] * $b[0][3],
                $a[0][1] * $b[0][0]
                    + $a[1][1] * $b[0][1]
                    + $a[2][1] * $b[0][2]
                    + $a[3][1] * $b[0][3],
                $a[0][2] * $b[0][0]
                    + $a[1][2] * $b[0][1]
                    + $a[2][2] * $b[0][2]
                    + $a[3][2] * $b[0][3],
                $a[0][3] * $b[0][0]
                    + $a[1][3] * $b[0][1]
                    + $a[2][3] * $b[0][2]
                    + $a[3][3] * $b[0][3],
            ],
            [
                $a[0][0] * $b[1][0]
                    + $a[1][0] * $b[1][1]
                    + $a[2][0] * $b[1][2]
                    + $a[3][0] * $b[1][3],
                $a[0][1] * $b[1][0]
                    + $a[1][1] * $b[1][1]
                    + $a[2][1] * $b[1][2]
                    + $a[3][1] * $b[1][3],
                $a[0][2] * $b[1][0]
                    + $a[1][2] * $b[1][1]
                    + $a[2][2] * $b[1][2]
                    + $a[3][2] * $b[1][3],
                $a[0][3] * $b[1][0]
                    + $a[1][3] * $b[1][1]
                    + $a[2][3] * $b[1][2]
                    + $a[3][3] * $b[1][3],
            ],
            [
                $a[0][0] * $b[2][0]
                    + $a[1][0] * $b[2][1]
                    + $a[2][0] * $b[2][2]
                    + $a[3][0] * $b[2][3],
                $a[0][1] * $b[2][0]
                    + $a[1][1] * $b[2][1]
                    + $a[2][1] * $b[2][2]
                    + $a[3][1] * $b[2][3],
                $a[0][2] * $b[2][0]
                    + $a[1][2] * $b[2][1]
                    + $a[2][2] * $b[2][2]
                    + $a[3][2] * $b[2][3],
                $a[0][3] * $b[2][0]
                    + $a[1][3] * $b[2][1]
                    + $a[2][3] * $b[2][2]
                    + $a[3][3] * $b[2][3],
            ],
            [
                $a[0][0] * $b[3][0]
                    + $a[1][0] * $b[3][1]
                    + $a[2][0] * $b[3][2]
                    + $a[3][0] * $b[3][3],
                $a[0][1] * $b[3][0]
                    + $a[1][1] * $b[3][1]
                    + $a[2][1] * $b[3][2]
                    + $a[3][1] * $b[3][3],
                $a[0][2] * $b[3][0]
                    + $a[1][2] * $b[3][1]
                    + $a[2][2] * $b[3][2]
                    + $a[3][2] * $b[3][3],
                $a[0][3] * $b[3][0]
                    + $a[1][3] * $b[3][1]
                    + $a[2][3] * $b[3][2]
                    + $a[3][3] * $b[3][3],
            ],
        ])
    };
}

#[macro_export]
macro_rules! cross {
    ($a:expr, $b:expr) => {
        crate::linalg::Vec3f::from([[
            $a.y() * $b.z() - $a.z() * $b.y(),
            $a.z() * $b.x() - $a.x() * $b.z(),
            $a.x() * $b.y() - $a.y() * $b.x(),
        ]])
    };
}

#[macro_export]
macro_rules! vec_normalized {
    ($a:expr) => {
        $a / $a.length()
    };
}

#[macro_export]
macro_rules! vec_angle {
    ($a:expr, $b:expr) => {
        a.dot(b) / (a.length_squared() * b.length_squared()).sqrt()
    };
}

#[macro_export]
macro_rules! vec_from {
    ($($a:expr), *) => {
        crate::linalg::TVec::from([[$($a), *]])
    }
}

#[macro_export]
macro_rules! qua_from {
    ($a:expr, $b:expr, $c:expr, $d:expr) => {
        crate::linalg::Qua::new_from($a, $b, $c, $d)
    };
}

#[macro_export]
macro_rules! cmp_from {
    ($a:expr, $b:expr) => {
        crate::linalg::Cmp::new_from($a, $b)
    };
}

#[macro_export]
macro_rules! lerp {
    ($a:expr, $b:expr, $c:expr) => {
        $a * (1.0 - $c) + $b * $c
    };
}
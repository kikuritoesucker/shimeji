mod linalg;
mod tween;
use glium;
use linalg::{TMat, Vec3f};
fn main() {
    // let mut myMat: linalg::TMat<f32, 4, 4> = Default::default();
    // let mut myMat: linalg::TMat<f32, 4, 4> = TMat::from(0.0);
    // for i in 0..4 {
    //     for j in 0..4 {
    //         myMat[i][j] = (i * 4 + j) as f32;
    //     }
    // }
    // println!("{myMat:?}");
    // let t1 = myMat.mm(&myMat.transpose());
    // let t2 = mat4f_mul!(myMat, myMat.transpose());
    // println!("{t1:?}\n{t2:?}");

    // let mut v1: Vec3f = Vec3f::new();
    // *v1.x_mut() = 1.0;
    // *v1.y_mut() = 2.0;
    // *v1.z_mut() = 3.0;
    // println!("{:?} · {:?} = {:?}", v1, v1, v1.dot(&v1));

    // let v1 = Vec3f::from([[1.0, 2.0, 3.0]]);
    // let v2 = Vec3f::from([[4.0, 5.0, 6.0]]);
    // println!("{:?}", v1.cross(&v2));
    // assert_eq!(v1.cross(&v2), vec3f_cross!(v1, v2));
}

mod linalg;
mod tween;

use linalg::*;
fn main() {
    let p1 = Mat4f::translation_xyz(1.0, 2.0, 3.0);
    let p2 = Vec4f::from([[0.0, 1.0, 2.0, 1.0]]);
    let p = p1.mm(&p2);
    println!("{:?}", p);
}

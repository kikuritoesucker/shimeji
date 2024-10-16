mod linalg;
mod tween;

use linalg::*;
fn main() {
    let q1 = vec_from!(1.0, 2.0, 3.0);
    let q2 = vec_from!(2.0, 4.0, 2.0);
    println!("{:?}", cross!(q1, q2).dot(&q2));
}

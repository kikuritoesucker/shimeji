mod linalg;
mod tween;

use linalg::*;
fn main() {
    let p = cmp_from!(1.0, 1.0);
    let q = cmp_from!(1.0, 0.5);

    println!("{:?}", p.inverse() * q * p);
}

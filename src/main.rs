mod linalg;
mod tween;
mod io;
mod event;
mod object;
use linalg::*;

fn main() {
    let p = cmp_from!(0.0, (std::f32::consts::PI));
    let p = p.exp();
    let p : Qua = p.into();
    println!("{:?}", p);
}

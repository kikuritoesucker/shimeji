mod linalg;
mod tween;
mod io;
mod event;
mod object;
use linalg::*;
use scalar::*;

fn main() {
    let p = cmp_from!(0.0, (std::f32::consts::PI));
    let p = p.exp();
    let p : Qua = p.into();

    // let mut myEvent: event::event_sync::Event<f32> = event :: event_sync ::Event::new();
    // myEvent.subscribe(Box::new(|a : f32|{println!("{}", a)}));
    // myEvent.emit(10.0);

    println!("{:?}", p);
}

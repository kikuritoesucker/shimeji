mod linalg;
mod tween;
mod io;
mod event;
mod node;
use std::{cell::RefCell, rc::Rc};

use linalg::*;
use node::*;
use scalar::*;

fn main() {
    // let root = Node::new(None);
    // root.borrow_mut().bind_method(Box::new(|caller|{println!("Root is called! I have {} children", caller.children.len())}));
    // let node1 = Node::new(Some(root.clone()));
    // let node2 = Node::new(Some(root.clone()));

    // node1.borrow_mut().bind_method(Box::new(|caller|{println!("Node1 is called!")}));
    // node2.borrow_mut().bind_method(Box::new(|caller|{println!("Node2 is called!")}));

    // let node3 = Node::new(Some(node1.clone()));
    // node3.borrow_mut().bind_method(Box::new(|_|{println!("I am a child of node1 and i have been called!")}));

    // root.borrow().process();

    let q1 = qua_from!(0.0, 1.0, 0.0, 0.0);
    let q2 = qua_from!(0.0, 0.0, 1.0, 0.0);
    for i in 0..=10 {
        let t = (i as f32) / 10_f32;
        let q_interpolated = q1.slerp(&q2, t);
        println!("{q_interpolated:?}, {}", q_interpolated.abs());
    }
}

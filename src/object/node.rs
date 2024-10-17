use core::panic;
use std::{cell::RefCell, rc::Rc};

pub struct Node {
    parent : Option<Rc<Node>>,
    children : Vec<Rc<Node>>,
    
    process : Option<Box<dyn Fn()>>
}

impl Node {
    fn new() -> Self {
        Self {
            parent : None,
            children : vec![],
            process : None
        }
    }

    fn add_child(&mut self, child : Rc<Node>) {
        self.children.push(child);
    }

    fn attach(&mut self, parent : &mut Self){
        unimplemented!()
    }
}


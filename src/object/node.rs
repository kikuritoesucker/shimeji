use core::panic;
use std::{borrow::BorrowMut, cell::RefCell, rc::Rc, vec};

struct NodeRaw {
    pub parent: Option<Rc<RefCell<NodeRaw>>>,
    pub children: Vec<Rc<RefCell<NodeRaw>>>,
    pub process: Option<Box<dyn Fn()>>,
}

pub struct Node(Rc<RefCell<NodeRaw>>);

impl Node {
    fn new() -> Self {
        Self(Rc::new(RefCell::new(NodeRaw {
            parent: None,
            children: Vec::new(),
            process: None,
        })))
    }

    fn add_child(&self, other: &Self) {
        let mut parentref = self.0.clone();
        unsafe{
            let parentnode = &mut *parentref.as_ptr();
            parentnode.children.push(other.0.clone());
               
        }
        
    }
}

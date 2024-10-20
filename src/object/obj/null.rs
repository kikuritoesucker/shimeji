use std::cell::RefCell;

use crate::node::{NodeTree, Node};

pub struct Null {
    method: RefCell<Option<Box<dyn Fn(&NodeTree)>>>,
}

impl Null {
    pub fn new() -> Self {
        Self { method: RefCell::new(None)}
    }
}

impl Node for Null {
    fn process(&self, caller_node: &NodeTree) {}

    fn bind_method(&self, method: Box<dyn Fn(&NodeTree)>) {
        self.method.replace(Some(method));
    }

    fn call_method(&self, caller_node: &NodeTree) {
        if let Some(ref method) = *self.method.borrow() {
            (method)(caller_node);
        }
    }
}

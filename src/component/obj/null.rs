use std::cell::RefCell;

use crate::node_tree::{Node, Processable};

pub struct Null {
    method: RefCell<Option<Box<dyn Fn(&Node)>>>,
}

impl Null {
    pub fn new() -> Self {
        Self { method: RefCell::new(None)}
    }
}

impl Processable for Null {
    fn process(&self, caller_node: &Node) {}

    fn bind_method(&self, method: Box<dyn Fn(&Node)>) {
        self.method.replace(Some(method));
    }

    fn call_method(&self, caller_node: &Node) {
        if let Some(ref method) = *self.method.borrow() {
            (method)(caller_node);
        }
    }
}

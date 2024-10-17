use core::panic;
use std::{cell::{RefCell, RefMut}, fmt::Debug, rc::Rc};
use super::{obj, obj_trait::*};

#[macro_export]
macro_rules! get_ref {
    ($a:expr) => {
        Rc::new(RefCell::new($a))
    };
}

pub struct Node {
    pub parent: Option<Rc<RefCell<Node>>>,
    pub children: Vec<Rc<RefCell<Node>>>,
    pub obj : Box<dyn Object>,
    //pub proc: Option<Box<dyn Fn(&Option<Box<dyn Object>>, &Node)>>,
}

impl Node {
    pub fn new(parent : Option<Rc<RefCell<Node>>>) -> Rc<RefCell<Node>>{
        match parent {
            Some(parent) => {
                let parent_node: Rc<RefCell<Node>> = parent.clone();
                let mut parent_node = (*parent_node).borrow_mut();
                let new_node = 
                Self{
                    parent : Some(parent.clone()),
                    children :Vec::new(),
                    obj : Box::new(obj::Null::new()),
                };
                let new_node = get_ref!(new_node);
                parent_node.add_child(new_node.clone());
                new_node
            },
            None => {
                let new_node = Self{
                    parent : None,
                    children :Vec::new(),
                    obj : Box::new(obj::Null::new())
                };
                get_ref!(new_node)
            }
        }
    }

    fn add_child(&mut self, other : Rc<RefCell<Node>>) {
        self.children.push(other);
    }

    pub fn bind_method(&self, method : Box<dyn Fn(&Node)>) -> &Self{
        self.obj.bind_method(method);
        self
    }

    pub fn bind_object(&mut self, object: Box<dyn Object>) -> &Self{
        self.obj = object;
        self
    }

    pub fn process(&self) {
        self.obj.process(&self);
        self.obj.call_method(&self);
        for child in &self.children {
            child.borrow().process();
        }
    }
}
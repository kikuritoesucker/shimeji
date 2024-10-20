use core::panic;
use std::{cell::{RefCell, RefMut}, fmt::Debug, rc::Rc};
use crate::object::obj::*;

#[macro_export]
macro_rules! get_ref {
    ($a:expr) => {
        Rc::new(RefCell::new($a))
    };
}

pub trait Node{
    fn process(&self, caller_node : &NodeTree);
    fn bind_method(&self, method : Box<dyn Fn(&NodeTree)>);
    fn call_method(&self, caller_node : &NodeTree);
}

pub struct NodeTree {
    pub parent: Option<Rc<RefCell<NodeTree>>>,
    pub children: Vec<Rc<RefCell<NodeTree>>>,
    pub obj : Box<dyn Node>,
    //pub proc: Option<Box<dyn Fn(&Option<Box<dyn Object>>, &Node)>>,
}

impl NodeTree {
    pub fn new(parent : Option<Rc<RefCell<NodeTree>>>) -> Rc<RefCell<NodeTree>>{
        match parent {
            Some(parent) => {
                let parent_node: Rc<RefCell<NodeTree>> = parent.clone();
                let mut parent_node = (*parent_node).borrow_mut();
                let new_node = 
                Self{
                    parent : Some(parent.clone()),
                    children :Vec::new(),
                    obj : Box::new(Null::new()),
                };
                let new_node = get_ref!(new_node);
                parent_node.add_child(new_node.clone());
                new_node
            },
            None => {
                let new_node = Self{
                    parent : None,
                    children :Vec::new(),
                    obj : Box::new(Null::new())
                };
                get_ref!(new_node)
            }
        }
    }

    fn add_child(&mut self, other : Rc<RefCell<NodeTree>>) {
        self.children.push(other);
    }

    pub fn bind_method(&self, method : Box<dyn Fn(&NodeTree)>) -> &Self{
        self.obj.bind_method(method);
        self
    }

    pub fn bind_object(&mut self, object: Box<dyn Node>) -> &Self{
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
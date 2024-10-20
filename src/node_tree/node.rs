use crate::component::obj::*;
use core::panic;
use std::sync::{LazyLock, Arc, Mutex};

macro_rules! wrap {
    ($a:expr) => {
        Arc::new(Mutex::new($a))
    };
}

pub trait Node {
    fn process(&self, caller_node: &NodeTree);
    fn bind_method(&self, method: Box<dyn Fn(&NodeTree)>);
    fn call_method(&self, caller_node: &NodeTree);
}

pub struct NodeTree {
    pub parent: Option<Arc<Mutex<NodeTree>>>,
    pub children: Vec<Arc<Mutex<NodeTree>>>,
    pub obj: Box<dyn Node>,

    //pub id : usize,
    //pub proc: Option<Box<dyn Fn(&Option<Box<dyn Object>>, &Node)>>,
}

impl NodeTree {
    pub fn new(parent: Option<Arc<Mutex<NodeTree>>>) -> Arc<Mutex<NodeTree>> {
        match parent {
            Some(parent) => {
                let parent_node: Arc<Mutex<NodeTree>> = parent.clone();
                let mut parent_node = parent_node.try_lock().unwrap();
                let new_node = Self {
                    parent: Some(parent.clone()),
                    children: Vec::new(),
                    //id : 
                    obj: Box::new(Null::new()),
                };
                let new_node = wrap!(new_node);
                parent_node.children.push(new_node.clone());
                new_node
            }
            None => {
                let new_node = Self {
                    parent: None,
                    children: Vec::new(),
                    obj: Box::new(Null::new()),
                };
                wrap!(new_node)
            }
        }
    }

    pub fn add_child(parent: Arc<Mutex<NodeTree>>, child: Arc<Mutex<NodeTree>>) {
        match &child.try_lock().unwrap().parent {
            Some(parent) => panic!("node already has a parent"),
            None => {
                child.try_lock().unwrap().parent = Some(parent.clone());
                parent.try_lock().unwrap().children.push(child.clone());
            }
        }
    
    }

    pub fn deattach(node: Arc<Mutex<NodeTree>>) -> Arc<Mutex<NodeTree>> {
        match &node.try_lock().unwrap().parent {
            None => (),
            Some(_) => {
                node.try_lock().unwrap().parent = None;
            }
        }
        node
    }

    pub fn bind_method(&self, method: Box<dyn Fn(&NodeTree)>) -> &Self {
        self.obj.bind_method(method);
        self
    }

    pub fn bind_object(&mut self, object: Box<dyn Node>) -> &Self {
        self.obj = object;
        self
    }

    pub fn process(&self) {
        self.obj.process(&self);
        self.obj.call_method(&self);
        for child in &self.children {
            child.try_lock().unwrap().process();
        }
    }
}
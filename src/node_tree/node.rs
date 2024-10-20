use lazy_static::lazy_static;

use crate::component::obj::*;
use core::panic;
use std::{cell::RefCell, rc::Rc, sync::Mutex};

macro_rules! wrap {
    ($a:expr) => {
        Rc::new(RefCell::new($a))
    };
}

macro_rules! node {
    ($a : expr) => {
        (*$a).borrow_mut()
    };
}

lazy_static! {
    static ref NODE_IDENT: Mutex<usize> = Mutex::new(0);
}

pub trait Processable {
    fn process(&self, caller_node: &Node);
    fn bind_method(&self, method: Box<dyn Fn(&Node)>);
    fn call_method(&self, caller_node: &Node);
}

/// Wrapper for NodeTree.
pub struct Node(Rc<RefCell<NodeTree>>);

impl Node {
    pub fn new(parent: Option<&Self>) -> Self {
        match parent {
            None => NodeTree::new(None),
            Some(p) => NodeTree::new(Some(p.0.clone()))
        }
    }

    pub fn add_child(&self, other: &Self) {
        if !self.is_successor_of(other){
            NodeTree::add_child(self.0.clone(), other.0.clone());
        } else {
            panic!("attempting to add an ancestor node as child");
        }
    }

    pub fn detach(&self) {
        NodeTree::detach(&self.0);
    }

    pub fn bind_method(&self, method: Box<dyn Fn(&Node)>) -> Self {
        Self(NodeTree::bind_method(&self.0, method).clone())
    }

    pub fn bind_object(&self, object: Box<dyn Processable>) -> Self {
        Self(NodeTree::bind_object(&self.0, object).clone())
    }

    pub fn is_successor_of(&self, other : &Self) -> bool {
        let mut node = self.0.clone();
        loop {
            if let Some(parent) = node!(node.clone()).parent.clone(){
                if Rc::ptr_eq(&parent, &other.0) {
                    return true;
                }
                node = parent;
            } else {
                break;
            }
            
        }
        false
    }

    pub fn is_sibling_with(&self, other : &Self) -> bool {
        if let (Some(p1), Some(p2)) = (node!(self.0).parent.clone(), node!(other.0).parent.clone()) {
            return Rc::ptr_eq(&p1, &p2);
        }
        false
    }

    pub fn process(&self) {
        NodeTree::process(&node!(self.0), &self);
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

struct NodeTree {
    pub parent: Option<Rc<RefCell<NodeTree>>>,
    pub children: Vec<Rc<RefCell<NodeTree>>>,
    pub obj: Box<dyn Processable>,
    pub id: usize,
    //pub proc: Option<Box<dyn Fn(&Option<Box<dyn Object>>, &Node)>>,
}

impl NodeTree {
    fn new(parent: Option<Rc<RefCell<NodeTree>>>) -> Node {
        match parent {
            Some(parent) => {
                let parent_node: Rc<RefCell<NodeTree>> = parent.clone();
                let mut parent_node = (*parent_node).borrow_mut();
                let new_node = Self {
                    parent: Some(parent.clone()),
                    children: Vec::new(),
                    id: {
                        let mut ident = NODE_IDENT.try_lock().unwrap();
                        *ident += 1;
                        *ident
                    },
                    obj: Box::new(Null::new()),
                };
                let new_node = wrap!(new_node);
                parent_node.children.push(new_node.clone());
                Node(new_node)
            }
            None => {
                let new_node = Self {
                    parent: None,
                    children: Vec::new(),
                    id: {
                        let mut ident = NODE_IDENT.try_lock().unwrap();
                        *ident += 1;
                        *ident
                    },
                    obj: Box::new(Null::new()),
                };
                Node(wrap!(new_node))
            }
        }
    }

    fn add_child(parent: Rc<RefCell<NodeTree>>, child: Rc<RefCell<NodeTree>>) {
        match &child.clone().borrow().parent {
            Some(parent) => panic!("node already has an parent"),
            None => {
                node!(parent).children.push(child.clone());
                node!(child).parent = Some(parent.clone());
            }
        }
    }

    fn detach(this: &Rc<RefCell<NodeTree>>) {
        if let Some(parent) = &node!(this).parent {
            let mut parent_borrowed = parent.borrow_mut();
            parent_borrowed
                .children
                .retain(|child| !Rc::ptr_eq(child, &this));
        }
        node!(this).parent = None;
    }

    fn bind_method(
        this: &Rc<RefCell<NodeTree>>,
        method: Box<dyn Fn(&Node)>,
    ) -> &Rc<RefCell<NodeTree>> {
        node!(this).obj.bind_method(method);
        this
    }

    fn bind_object(
        this: &Rc<RefCell<NodeTree>>,
        object: Box<dyn Processable>,
    ) -> &Rc<RefCell<NodeTree>> {
        node!(this).obj = object;
        this
    }

    fn process(&self, caller: &Node) {
        self.obj.process(&caller);
        self.obj.call_method(&caller);
        for child in &self.children {
            child.borrow().process(caller);
        }
    }
}

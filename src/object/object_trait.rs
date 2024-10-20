use std::rc::Rc;

use super::Node;

pub trait Object{
    fn process(&self, caller_node : &Node);
    fn bind_method(&self, method : Box<dyn Fn(&Node)>);
    fn call_method(&self, caller_node : &Node);
}
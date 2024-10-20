use crate::node::Node;

pub trait Drawable : Node{
    fn draw(&self);
}
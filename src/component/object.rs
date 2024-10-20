use crate::node_tree::Node;

pub trait Object : Node {
    fn draw(&self);

    fn free(&self);
    fn free_safe(&self);
}
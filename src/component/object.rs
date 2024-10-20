use crate::node_tree::Processable;

pub trait Object : Processable {
    fn draw(&self);

    fn free(&self);
    fn free_safe(&self);
}
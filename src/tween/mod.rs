/*
    Implementation of tweener.

    A tween function takes an input value `x` ranging from 0 to 1 and maps it according to a specific procedure.
    It ensures that the function satisfies the following conditions:
    - f(0) = 0
    - f(1) = 1

    Tween functions are commonly used in animations to interpolate values smoothly over time.
*/
#![allow(unused)]

mod tweener;

pub use tweener::*;

pub struct Tween<'a> {
    //pub value: f32,
    pub tween_function: &'a dyn Fn(f32) -> f32,
}

impl<'a> Tween<'a> {
    pub fn new(tween_function: &'a dyn Fn(f32) -> f32) -> Self {
        Self {
            //value: 0.0,
            tween_function,
        }
    }

    pub fn get(&self, value : f32) -> f32{
        (self.tween_function)(value)
    }
}

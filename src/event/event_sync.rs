#![allow(unused)]

use std::collections::HashMap;
use std::rc::Rc;

type Callback<T> = Box<dyn Fn(T)>;

pub struct Event<T> {
    callbacks : HashMap<usize, Callback<T>>,
    next_id : usize
}

impl<T: Clone> Event<T> {
    pub fn new() -> Self {
        Self {
            callbacks : HashMap ::new(),
            next_id : 0
        }
    }
    pub fn subscribe(&mut self, callback: Callback<T>) -> usize {
        self.callbacks.insert(self.next_id, callback);
        self.next_id += 1;
        self.next_id
    }

    pub fn unsubscribe(&mut self, id: usize) {
        self.callbacks.remove(&id);
    }

    pub fn emit(&self, data: T) {
        for callback in self.callbacks.values() {
            callback(data.clone());
        }
    }
}

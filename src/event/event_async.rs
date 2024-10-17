#![allow(unused)]

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

type Callback<T> = Box<dyn Fn(T) + Send + Sync>;

pub struct EventAsync<T> {
    callbacks: Arc<Mutex<HashMap<usize, Callback<T>>>>,
    next_id: Arc<Mutex<usize>>,
}

impl<T: Clone> EventAsync<T> {
    pub fn new() -> Self {
        EventAsync {
            callbacks: Arc::new(Mutex::new(HashMap::new())),
            next_id: Arc::new(Mutex::new(0)),
        }
    }

    pub fn subscribe(&self, callback: Callback<T>) -> usize {
        let mut callbacks = self.callbacks.lock().unwrap();
        let mut next_id = self.next_id.lock().unwrap();
        let id = *next_id;
        callbacks.insert(id, callback);
        *next_id += 1;
        id
    }

    pub fn unsubscribe(&self, id: usize) {
        let mut callbacks = self.callbacks.lock().unwrap();
        callbacks.remove(&id);
    }

    pub fn emit(&self, data: T) {
        let callbacks = self.callbacks.lock().unwrap();
        for callback in callbacks.values() {
            callback(data.clone());
        }
    }
}
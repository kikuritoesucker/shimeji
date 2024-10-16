use std::collections::HashMap;
use std::rc::Rc;

type Callback<T> = Box<dyn Fn(T)>;

pub struct Event<T> {
    callbacks : HashMap<usize, Callback<T>>,
    next_id : usize
}

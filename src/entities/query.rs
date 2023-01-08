use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Query {}

impl Query {
    pub fn new() -> Self {
        Self {}
    }
    pub fn with_component<T: Any>(&mut self) -> &mut Self {
        self
    }

    pub fn run(&self) -> Vec<Vec<Rc<RefCell<dyn Any>>>> {
        vec![]
    }
}
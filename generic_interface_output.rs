use std::collections::HashMap;


pub trait Container<T> {
    fn get_value(&self) -> T;
    fn set_value(&mut self, value: T);
    fn getValue(&self, ) -> T;
}



use std::collections::HashMap;


pub trait SimpleInterface {
    fn get_value(&self) -> f64;
    fn set_value(&mut self, value: f64);
    fn getValue(&self, ) -> f64;
}



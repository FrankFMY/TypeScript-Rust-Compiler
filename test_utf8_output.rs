use std::collections::HashMap;


pub trait ТестИнтерфейс {
    fn get_имя(&self) -> String;
    fn set_имя(&mut self, value: String);
    fn get_возраст(&self) -> f64;
    fn set_возраст(&mut self, value: f64);
}



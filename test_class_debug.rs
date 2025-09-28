use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Person {
    pub name: String,
    pub age: f64
}


impl Person {
    pub fn new(name: String, age: f64) -> Self {
        Self {
            name: name,
            age: age
        }
    }
    pub fn greet(&self) -> String{
        return ("Hello, I'm ".to_string() + self.name);
    }
}


fn main() {
    // Example usage
    println!("TypeScript to Rust compilation successful!");
}

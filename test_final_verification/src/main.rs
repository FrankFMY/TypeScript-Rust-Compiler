use test_final_verification::SimpleClass;

fn main() {
    let instance = SimpleClass::new();
    println!("Created SimpleClass with prop1: {}", instance.prop1);
    println!("Method result: {}", instance.method1());
    println!("TypeScript to Rust compilation successful!");
}

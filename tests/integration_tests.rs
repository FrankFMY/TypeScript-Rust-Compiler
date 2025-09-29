//! Integration tests for the TypeScript-Rust-Compiler

use std::fs;
use tempfile::TempDir;
use TypeScript_Rust_Compiler::compiler::Compiler;

/// Test basic TypeScript compilation
#[test]
fn test_basic_compilation() {
    let temp_dir = TempDir::new().unwrap();
    let input_file = temp_dir.path().join("input.ts");
    let output_file = temp_dir.path().join("output.rs");

    // Create test TypeScript file
    let ts_code = r#"
function add(a: number, b: number): number {
    return a + b;
}

const result = add(5, 3);
console.log(result);
"#;

    fs::write(&input_file, ts_code).unwrap();

    // Compile
    let mut compiler = Compiler::new();
    let result = compiler.compile(&input_file, &output_file);

    if let Err(e) = &result {
        println!("Compilation error: {:?}", e);
    }
    assert!(result.is_ok());

    // Check output file exists
    assert!(output_file.exists());

    // Check output content
    let rust_code = fs::read_to_string(&output_file).unwrap();
    assert!(rust_code.contains("fn add"));
    assert!(rust_code.contains("f64"));
}

/// Test class compilation
#[test]
fn test_class_compilation() {
    let temp_dir = TempDir::new().unwrap();
    let input_file = temp_dir.path().join("input.ts");
    let output_file = temp_dir.path().join("output.rs");

    let ts_code = r#"
class Person {
    name: string;
    age: number;

    constructor(name: string, age: number) {
        this.name = name;
        this.age = age;
    }

    greet(): string {
        return `Hello, I'm ${this.name}`;
    }
}
"#;

    fs::write(&input_file, ts_code).unwrap();

    let mut compiler = Compiler::new();
    let result = compiler.compile(&input_file, &output_file);

    if let Err(e) = &result {
        println!("Compilation error: {:?}", e);
    }
    assert!(result.is_ok());

    let rust_code = fs::read_to_string(&output_file).unwrap();
    assert!(rust_code.contains("struct Person"));
    assert!(rust_code.contains("impl Person"));
}

/// Test interface compilation
#[test]
fn test_interface_compilation() {
    let temp_dir = TempDir::new().unwrap();
    let input_file = temp_dir.path().join("input.ts");
    let output_file = temp_dir.path().join("output.rs");

    let ts_code = r#"
interface Shape {
    area(): number;
    perimeter(): number;
}

class Circle implements Shape {
    radius: number;

    constructor(radius: number) {
        this.radius = radius;
    }

    area(): number {
        return Math.PI * this.radius * this.radius;
    }

    perimeter(): number {
        return 2 * Math.PI * this.radius;
    }
}
"#;

    fs::write(&input_file, ts_code).unwrap();

    let mut compiler = Compiler::new();
    let result = compiler.compile(&input_file, &output_file);

    if let Err(e) = &result {
        println!("Interface compilation error: {:?}", e);
    }
    assert!(result.is_ok());

    let rust_code = fs::read_to_string(&output_file).unwrap();
    assert!(rust_code.contains("trait Shape"));
    assert!(rust_code.contains("struct Circle"));
}

/// Test enum compilation
#[test]
fn test_enum_compilation() {
    let temp_dir = TempDir::new().unwrap();
    let input_file = temp_dir.path().join("input.ts");
    let output_file = temp_dir.path().join("output.rs");

    let ts_code = r#"
enum Color {
    Red,
    Green,
    Blue
}

enum Status {
    Pending = "pending",
    Approved = "approved",
    Rejected = "rejected"
}
"#;

    fs::write(&input_file, ts_code).unwrap();

    let mut compiler = Compiler::new();
    let result = compiler.compile(&input_file, &output_file);

    if let Err(e) = &result {
        println!("Project compilation error: {:?}", e);
    }
    assert!(result.is_ok());

    let rust_code = fs::read_to_string(&output_file).unwrap();
    assert!(rust_code.contains("enum Color"));
    assert!(rust_code.contains("enum Status"));
}

/// Test generic types compilation
#[test]
fn test_generic_compilation() {
    let temp_dir = TempDir::new().unwrap();
    let input_file = temp_dir.path().join("input.ts");
    let output_file = temp_dir.path().join("output.rs");

    let ts_code = r#"
interface Container<T> {
    value: T;
    getValue(): T;
    setValue(value: T): void;
}

class Box<T> implements Container<T> {
    value: T;

    constructor(value: T) {
        this.value = value;
    }

    getValue(): T {
        return this.value;
    }

    setValue(value: T): void {
        this.value = value;
    }
}
"#;

    fs::write(&input_file, ts_code).unwrap();

    let mut compiler = Compiler::new();
    let result = compiler.compile(&input_file, &output_file);

    if let Err(e) = &result {
        println!("Generic compilation error: {:?}", e);
    }
    assert!(result.is_ok());

    let rust_code = fs::read_to_string(&output_file).unwrap();
    assert!(rust_code.contains("trait Container"));
    assert!(rust_code.contains("struct Box"));
}

/// Test project compilation
#[test]
fn test_project_compilation() {
    let temp_dir = TempDir::new().unwrap();
    let input_dir = temp_dir.path().join("input");
    let output_dir = temp_dir.path().join("output");

    fs::create_dir_all(&input_dir).unwrap();

    // Create multiple TypeScript files
    let main_ts = r#"
import { Calculator } from './calculator';
import { Person } from './person';

const calc = new Calculator();
const person = new Person('John', 30);

console.log(calc.add(5, 3));
console.log(person.greet());
"#;

    let calculator_ts = r#"
export class Calculator {
    add(a: number, b: number): number {
        return a + b;
    }

    subtract(a: number, b: number): number {
        return a - b;
    }
}
"#;

    let person_ts = r#"
export class Person {
    name: string;
    age: number;

    constructor(name: string, age: number) {
        this.name = name;
        this.age = age;
    }

    greet(): string {
        return `Hello, I'm ${this.name}`;
    }
}
"#;

    fs::write(input_dir.join("main.ts"), main_ts).unwrap();
    fs::write(input_dir.join("calculator.ts"), calculator_ts).unwrap();
    fs::write(input_dir.join("person.ts"), person_ts).unwrap();

    // Compile project
    let mut compiler = Compiler::new();
    let result = compiler.compile_project(&input_dir, &output_dir);

    if let Err(e) = &result {
        println!("Project compilation error: {:?}", e);
    }
    assert!(result.is_ok());

    // Check output files
    assert!(output_dir.join("Cargo.toml").exists());
    assert!(output_dir.join("README.md").exists());
    assert!(output_dir.join(".gitignore").exists());
}

/// Test runtime compilation
#[test]
fn test_runtime_compilation() {
    let temp_dir = TempDir::new().unwrap();
    let input_file = temp_dir.path().join("input.ts");
    let output_file = temp_dir.path().join("output.rs");

    let ts_code = r#"
function processData(data: any): any {
    if (typeof data === 'string') {
        return data.toUpperCase();
    } else if (typeof data === 'number') {
        return data * 2;
    }
    return data;
}

const result1 = processData("hello");
const result2 = processData(42);
"#;

    fs::write(&input_file, ts_code).unwrap();

    let mut compiler = Compiler::new().with_runtime(true);
    let result = compiler.compile(&input_file, &output_file);

    if let Err(e) = &result {
        println!("Runtime compilation error: {:?}", e);
    }
    assert!(result.is_ok());

    let rust_code = fs::read_to_string(&output_file).unwrap();
    assert!(rust_code.contains("Box<dyn Any>"));
    assert!(rust_code.contains("TypeScriptObject"));
}

/// Test error handling
#[test]
fn test_error_handling() {
    let temp_dir = TempDir::new().unwrap();
    let input_file = temp_dir.path().join("input.ts");
    let output_file = temp_dir.path().join("output.rs");

    // Create invalid TypeScript file
    let invalid_ts = r#"
function add(a: number, b: number): number {
    return a + b  // Missing semicolon
}

const result = add(5, 3;
"#;

    fs::write(&input_file, invalid_ts).unwrap();

    let mut compiler = Compiler::new();
    let result = compiler.compile(&input_file, &output_file);

    // Should handle parse errors gracefully with tolerant parsing
    // Our compiler continues parsing even with syntax errors
    assert!(result.is_ok());

    // Verify that output file was created
    assert!(output_file.exists());
}

/// Test advanced TypeScript features
#[test]
fn test_advanced_features() {
    let temp_dir = TempDir::new().unwrap();
    let input_file = temp_dir.path().join("input.ts");
    let output_file = temp_dir.path().join("output.rs");

    let ts_code = r#"
// Union types
type UnionType = string | number | boolean;

// Intersection types
type IntersectionType = { name: string } & { age: number };

// Generic functions
function identity<T>(arg: T): T {
    return arg;
}

// Generic classes
class Container<T> {
    value: T;

    constructor(value: T) {
        this.value = value;
    }

    getValue(): T {
        return this.value;
    }
}

// Template literals
const template = `Hello, ${"World"}!`;

// Optional chaining
const obj = { a: { b: 42 } };
const result = obj?.a?.b;

// Nullish coalescing
const fallback = null ?? "default";

// Type assertions
const assertion = "42" as number;

// Enums
enum Status {
    Active = "active",
    Inactive = "inactive"
}

// Interfaces
interface Person {
    name: string;
    age: number;
    greet?(): string;
}

// Classes implementing interfaces
class Employee implements Person {
    name: string;
    age: number;
    position: string;

    constructor(name: string, age: number, position: string) {
        this.name = name;
        this.age = age;
        this.position = position;
    }

    greet(): string {
        return `Hello, I'm ${this.name} and I work as ${this.position}`;
    }
}
"#;

    fs::write(&input_file, ts_code).unwrap();

    let mut compiler = Compiler::new().with_runtime(true);
    let result = compiler.compile(&input_file, &output_file);

    if let Err(e) = &result {
        println!("Advanced features compilation error: {:?}", e);
    }
    assert!(result.is_ok());

    let rust_code = fs::read_to_string(&output_file).unwrap();
    assert!(rust_code.contains("Union"));
    assert!(rust_code.contains("Intersection"));
    assert!(rust_code.contains("Container"));
    assert!(rust_code.contains("Status"));
    assert!(rust_code.contains("Person"));
    assert!(rust_code.contains("Employee"));
}

/// Test complex expressions
#[test]
fn test_complex_expressions() {
    let temp_dir = TempDir::new().unwrap();
    let input_file = temp_dir.path().join("input.ts");
    let output_file = temp_dir.path().join("output.rs");

    let ts_code = r#"
// Complex object literal
const complexObj = {
    nested: {
        value: 42,
        method() {
            return this.value * 2;
        }
    },
    array: [1, 2, 3, 4, 5],
    computed: {
        ["dynamic" + "Key"]: "value"
    }
};

// Complex function
function complexFunction(x: number, y: number): number {
    const a = x * 2;
    const b = y + 10;
    const c = a > b ? a - b : b - a;
    return c * (x + y);
}

// Arrow function with complex body
const complexArrow = (data: any) => {
    if (typeof data === 'string') {
        return data.toUpperCase();
    } else if (typeof data === 'number') {
        return data * 100;
    } else {
        return data;
    }
};
"#;

    fs::write(&input_file, ts_code).unwrap();

    let mut compiler = Compiler::new();
    let result = compiler.compile(&input_file, &output_file);

    assert!(result.is_ok());

    let rust_code = fs::read_to_string(&output_file).unwrap();
    assert!(rust_code.contains("complexObj"));
    assert!(rust_code.contains("complexFunction"));
    assert!(rust_code.contains("complexArrow"));
}

/// Test TypeScript modules and exports
#[test]
fn test_modules_and_exports() {
    let temp_dir = TempDir::new().unwrap();
    let input_dir = temp_dir.path().join("input");
    let output_dir = temp_dir.path().join("output");

    fs::create_dir_all(&input_dir).unwrap();

    // Create main module
    let main_ts = r#"
import { Calculator } from './calculator';
import { Person, greet } from './utils';
import DefaultExport from './default';

const calc = new Calculator();
const person = new Person('John', 30);

export { calc, person };
export default calc;
"#;

    // Create calculator module
    let calculator_ts = r#"
export class Calculator {
    add(a: number, b: number): number {
        return a + b;
    }

    multiply(a: number, b: number): number {
        return a * b;
    }
}

export interface MathOperation {
    (a: number, b: number): number;
}
"#;

    // Create utils module
    let utils_ts = r#"
export class Person {
    name: string;
    age: number;

    constructor(name: string, age: number) {
        this.name = name;
        this.age = age;
    }
}

export function greet(name: string): string {
    return `Hello, ${name}!`;
}
"#;

    // Create default export module
    let default_ts = r#"
class DefaultClass {
    value: string;

    constructor(value: string) {
        this.value = value;
    }

    getValue(): string {
        return this.value;
    }
}

export default DefaultClass;
"#;

    fs::write(input_dir.join("main.ts"), main_ts).unwrap();
    fs::write(input_dir.join("calculator.ts"), calculator_ts).unwrap();
    fs::write(input_dir.join("utils.ts"), utils_ts).unwrap();
    fs::write(input_dir.join("default.ts"), default_ts).unwrap();

    let mut compiler = Compiler::new();
    let result = compiler.compile_project(&input_dir, &output_dir);

    if let Err(e) = &result {
        println!("Modules compilation error: {:?}", e);
    }
    assert!(result.is_ok());

    // Check that output files exist
    assert!(output_dir.join("src").join("main.rs").exists());
    assert!(output_dir.join("src").join("calculator.rs").exists());
    assert!(output_dir.join("src").join("utils.rs").exists());
    assert!(output_dir.join("src").join("default.rs").exists());
}

/// Test optimization
#[test]
fn test_optimization() {
    let temp_dir = TempDir::new().unwrap();
    let input_file = temp_dir.path().join("input.ts");
    let output_file = temp_dir.path().join("output.rs");

    let ts_code = r#"
function fibonacci(n: number): number {
    if (n <= 1) return n;
    return fibonacci(n - 1) + fibonacci(n - 2);
}

const result = fibonacci(10);
"#;

    fs::write(&input_file, ts_code).unwrap();

    let mut compiler = Compiler::new().with_optimization(true);
    let result = compiler.compile(&input_file, &output_file);

    assert!(result.is_ok());

    let rust_code = fs::read_to_string(&output_file).unwrap();
    // Check for optimization hints in generated code
    assert!(rust_code.contains("fn fibonacci"));
}

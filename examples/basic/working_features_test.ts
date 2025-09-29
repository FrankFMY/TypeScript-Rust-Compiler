// TypeScript constructs that CURRENTLY work in the compiler

// ✅ VARIABLES
let message: string = "Hello, TypeScript!";
let count: number = 42;
let isActive: boolean = true;

// ✅ ARRAYS
let numbers: number[] = [1, 2, 3, 4, 5];
let strings: Array<string> = ["a", "b", "c"];

// ✅ OBJECTS
let person: { name: string; age: number } = { name: "John", age: 30 };

// ✅ INTERFACES (generate traits)
interface User {
    name: string;
    age: number;
}

// ✅ CLASSES (generate structs + impl blocks)
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

// ✅ FUNCTIONS
function add(a: number, b: number): number {
    return a + b;
}

function multiply(x: number, y: number): number {
    return x * y;
}

// ✅ ENUMS (generate enums + consts for string values)
enum Color {
    Red,
    Green,
    Blue
}

enum Status {
    Active = "active",
    Inactive = "inactive"
}

// ✅ TYPE ALIASES
type ID = string | number;
type Callback = () => void;

// Usage examples
const user: User = { name: "Alice", age: 25 };
const personInstance = new Person("Bob", 30);
const result = add(5, 3);
const color: Color = Color.Red;
const status: Status = Status.Active;

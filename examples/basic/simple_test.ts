// Simple TypeScript file that actually works
let message: string = 'Hello, TypeScript!';
let count: number = 42;
let isActive: boolean = true;
let numbers: number[] = [1, 2, 3, 4, 5];
let data: { name: string; age: number } = { name: 'John', age: 30 };

// This is what currently gets compiled to Rust
console.log(message);
console.log('Count:', count);
console.log('Active:', isActive);

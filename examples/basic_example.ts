// Basic TypeScript example for TS2RS compiler

// Simple function
function greet(name: string): string {
	return `Hello, ${name}!`;
}

// Function with multiple parameters
function add(a: number, b: number): number {
	return a + b;
}

// Function with optional parameter
function createUser(name: string, age?: number): User {
	return {
		name,
		age: age || 0,
	};
}

// Interface definition
interface User {
	name: string;
	age: number;
	email?: string;
}

// Class definition
class Calculator {
	private result: number;

	constructor(initialValue: number = 0) {
		this.result = initialValue;
	}

	add(value: number): Calculator {
		this.result += value;
		return this;
	}

	subtract(value: number): Calculator {
		this.result -= value;
		return this;
	}

	multiply(value: number): Calculator {
		this.result *= value;
		return this;
	}

	divide(value: number): Calculator {
		if (value === 0) {
			throw new Error('Division by zero');
		}
		this.result /= value;
		return this;
	}

	getResult(): number {
		return this.result;
	}

	reset(): void {
		this.result = 0;
	}
}

// Generic class
class Container<T> {
	private items: T[];

	constructor() {
		this.items = [];
	}

	add(item: T): void {
		this.items.push(item);
	}

	get(index: number): T | undefined {
		return this.items[index];
	}

	getAll(): T[] {
		return [...this.items];
	}

	size(): number {
		return this.items.length;
	}
}

// Enum
enum Status {
	Pending = 'pending',
	Approved = 'approved',
	Rejected = 'rejected',
}

// Type alias
type ID = string | number;

// Union type
type StringOrNumber = string | number;

// Intersection type
type UserWithId = User & { id: ID };

// Function with generic type
function identity<T>(value: T): T {
	return value;
}

// Async function
async function fetchUser(id: string): Promise<User> {
	// Simulate API call
	return new Promise((resolve) => {
		setTimeout(() => {
			resolve({
				name: 'John Doe',
				age: 30,
				email: 'john@example.com',
			});
		}, 1000);
	});
}

// Usage examples
const user = createUser('Alice', 25);
const calculator = new Calculator(10);
const container = new Container<string>();

calculator.add(5).multiply(2);
console.log(calculator.getResult()); // 30

container.add('item1');
container.add('item2');
console.log(container.size()); // 2

const status = Status.Approved;
console.log(status); // "approved"

const result = identity<string>('Hello, World!');
console.log(result); // "Hello, World!"

// Export for use in other modules
export { greet, add, createUser, Calculator, Container, Status, fetchUser };
export type { User, ID, StringOrNumber, UserWithId };

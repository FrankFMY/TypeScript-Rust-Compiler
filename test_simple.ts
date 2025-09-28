// Simple TypeScript test
interface Person {
	name: string;
	age: number;
}

class Calculator {
	add(a: number, b: number): number {
		return a + b;
	}
}

function greet(name: string): string {
	return `Hello, ${name}!`;
}

const person: Person = {
	name: 'John',
	age: 30,
};

const calc = new Calculator();
const result = calc.add(5, 3);
const message = greet(person.name);

console.log(message, 'Result:', result);

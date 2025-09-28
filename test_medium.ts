// Medium complexity TypeScript test
interface Person {
	name: string;
	age: number;
	email: string;
}

class Database {
	private data: Person[] = [];

	add(person: Person): void {
		this.data.push(person);
	}

	find(name: string): Person | null {
		return this.data.find((p) => p.name === name) || null;
	}

	getAll(): Person[] {
		return this.data;
	}
}

function createPerson(name: string, age: number, email: string): Person {
	return { name, age, email };
}

const db = new Database();
const person = createPerson('Alice', 25, 'alice@example.com');
db.add(person);

const found = db.find('Alice');
if (found) {
	console.log('Found:', found.name, found.age);
}

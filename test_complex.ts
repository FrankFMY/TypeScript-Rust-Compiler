// Complex TypeScript test
interface User {
	id: number;
	name: string;
	email: string;
}

class UserService {
	private users: User[] = [];

	addUser(user: User): void {
		this.users.push(user);
	}

	getUser(id: number): User | null {
		return this.users.find((u) => u.id === id) || null;
	}

	getAllUsers(): User[] {
		return this.users;
	}
}

function createUser(name: string, email: string): User {
	return {
		id: Date.now(),
		name,
		email,
	};
}

const service = new UserService();
const user = createUser('John', 'john@example.com');
service.addUser(user);

const foundUser = service.getUser(user.id);
if (foundUser) {
	console.log('Found user:', foundUser.name);
}

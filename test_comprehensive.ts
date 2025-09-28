// Comprehensive TypeScript test file
interface User {
	id: number;
	name: string;
	email: string;
	isActive: boolean;
	roles: string[];
	profile?: {
		avatar: string;
		bio: string;
	};
}

class UserService {
	private users: User[] = [];

	constructor(private apiUrl: string) {}

	async getUser(id: number): Promise<User | null> {
		const user = this.users.find((u) => u.id === id);
		return user || null;
	}

	async createUser(userData: Omit<User, 'id'>): Promise<User> {
		const newUser: User = {
			id: Date.now(),
			...userData,
		};
		this.users.push(newUser);
		return newUser;
	}

	updateUser(id: number, updates: Partial<User>): boolean {
		const userIndex = this.users.findIndex((u) => u.id === id);
		if (userIndex === -1) return false;

		this.users[userIndex] = { ...this.users[userIndex], ...updates };
		return true;
	}

	deleteUser(id: number): boolean {
		const initialLength = this.users.length;
		this.users = this.users.filter((u) => u.id !== id);
		return this.users.length < initialLength;
	}

	getActiveUsers(): User[] {
		return this.users.filter((u) => u.isActive);
	}

	getUserCount(): number {
		return this.users.length;
	}
}

enum UserRole {
	ADMIN = 'admin',
	USER = 'user',
	MODERATOR = 'moderator',
}

type UserWithRole = User & {
	role: UserRole;
	permissions: string[];
};

interface ApiResponse<T> {
	data: T;
	success: boolean;
	message?: string;
	errors?: string[];
}

class ApiClient {
	private baseUrl: string;

	constructor(baseUrl: string) {
		this.baseUrl = baseUrl;
	}

	async request<T>(
		endpoint: string,
		options: RequestInit = {}
	): Promise<ApiResponse<T>> {
		try {
			const response = await fetch(`${this.baseUrl}${endpoint}`, {
				headers: {
					'Content-Type': 'application/json',
					...options.headers,
				},
				...options,
			});

			if (!response.ok) {
				throw new Error(
					`HTTP ${response.status}: ${response.statusText}`
				);
			}

			const data = await response.json();
			return {
				data,
				success: true,
			};
		} catch (error) {
			return {
				data: null as T,
				success: false,
				message:
					error instanceof Error ? error.message : 'Unknown error',
				errors: [
					error instanceof Error ? error.message : 'Unknown error',
				],
			};
		}
	}
}

// Generic utility functions
function createApiResponse<T>(
	data: T,
	success: boolean = true,
	message?: string
): ApiResponse<T> {
	return { data, success, message };
}

function isUser(obj: any): obj is User {
	return (
		obj &&
		typeof obj.id === 'number' &&
		typeof obj.name === 'string' &&
		typeof obj.email === 'string' &&
		typeof obj.isActive === 'boolean' &&
		Array.isArray(obj.roles)
	);
}

// Advanced TypeScript features
type UserKeys = keyof User;
type UserValues = User[UserKeys];
type PartialUser = Partial<User>;
type RequiredUser = Required<User>;
type UserEmail = Pick<User, 'email'>;
type UserWithoutId = Omit<User, 'id'>;

// Conditional types
type NonNullable<T> = T extends null | undefined ? never : T;
type ApiResult<T> = T extends string ? { message: T } : { data: T };

// Mapped types
type ReadonlyUser = {
	readonly [K in keyof User]: User[K];
};

type OptionalUser = {
	[K in keyof User]?: User[K];
};

// Template literal types
type EventName<T extends string> = `on${Capitalize<T>}`;
type UserEvent = EventName<'click' | 'hover' | 'focus'>;

// Utility functions with generics
function mapArray<T, U>(array: T[], mapper: (item: T) => U): U[] {
	return array.map(mapper);
}

function filterArray<T>(array: T[], predicate: (item: T) => boolean): T[] {
	return array.filter(predicate);
}

function reduceArray<T, U>(
	array: T[],
	reducer: (acc: U, item: T) => U,
	initial: U
): U {
	return array.reduce(reducer, initial);
}

// Async/await examples
async function fetchUserData(userId: number): Promise<User | null> {
	const apiClient = new ApiClient('https://api.example.com');
	const response = await apiClient.request<User>(`/users/${userId}`);

	if (response.success && response.data) {
		return response.data;
	}

	return null;
}

async function processUsers(userIds: number[]): Promise<User[]> {
	const promises = userIds.map((id) => fetchUserData(id));
	const results = await Promise.all(promises);
	return results.filter((user): user is User => user !== null);
}

// Error handling
class ValidationError extends Error {
	constructor(message: string, public field: string) {
		super(message);
		this.name = 'ValidationError';
	}
}

function validateUser(user: any): user is User {
	if (!user || typeof user !== 'object') {
		throw new ValidationError('User must be an object', 'user');
	}

	if (typeof user.id !== 'number' || user.id <= 0) {
		throw new ValidationError('User ID must be a positive number', 'id');
	}

	if (typeof user.name !== 'string' || user.name.trim().length === 0) {
		throw new ValidationError(
			'User name must be a non-empty string',
			'name'
		);
	}

	if (typeof user.email !== 'string' || !user.email.includes('@')) {
		throw new ValidationError(
			'User email must be a valid email address',
			'email'
		);
	}

	return true;
}

// Main execution
async function main(): Promise<void> {
	try {
		const userService = new UserService('https://api.example.com');

		// Create a new user
		const newUser = await userService.createUser({
			name: 'John Doe',
			email: 'john@example.com',
			isActive: true,
			roles: ['user'],
		});

		console.log('Created user:', newUser);

		// Update user
		const updated = userService.updateUser(newUser.id, {
			name: 'John Smith',
			roles: ['user', 'premium'],
		});

		console.log('User updated:', updated);

		// Get user count
		const count = userService.getUserCount();
		console.log('Total users:', count);

		// Process multiple users
		const userIds = [1, 2, 3, 4, 5];
		const users = await processUsers(userIds);
		console.log('Processed users:', users.length);
	} catch (error) {
		console.error('Error in main:', error);
	}
}

// Export everything
export {
	User,
	UserService,
	UserRole,
	UserWithRole,
	ApiResponse,
	ApiClient,
	createApiResponse,
	isUser,
	fetchUserData,
	processUsers,
	ValidationError,
	validateUser,
	main,
};

export type {
	UserKeys,
	UserValues,
	PartialUser,
	RequiredUser,
	UserEmail,
	UserWithoutId,
	NonNullable,
	ApiResult,
	ReadonlyUser,
	OptionalUser,
	EventName,
	UserEvent,
};

// Advanced TypeScript example showcasing complex features

// Decorators (experimental)
function log(target: any, propertyKey: string, descriptor: PropertyDescriptor) {
	const originalMethod = descriptor.value;
	descriptor.value = function (...args: any[]) {
		console.log(`Calling ${propertyKey} with args:`, args);
		const result = originalMethod.apply(this, args);
		console.log(`Result:`, result);
		return result;
	};
}

// Class with decorators
class MathService {
	@log
	static add(a: number, b: number): number {
		return a + b;
	}

	@log
	static multiply(a: number, b: number): number {
		return a * b;
	}
}

// Complex generic types
interface Repository<T, K = string> {
	findById(id: K): Promise<T | null>;
	save(entity: T): Promise<T>;
	delete(id: K): Promise<boolean>;
	findAll(): Promise<T[]>;
}

// Abstract class
abstract class BaseEntity {
	protected id: string;
	protected createdAt: Date;

	constructor(id: string) {
		this.id = id;
		this.createdAt = new Date();
	}

	abstract validate(): boolean;
	abstract serialize(): object;

	getId(): string {
		return this.id;
	}

	getCreatedAt(): Date {
		return this.createdAt;
	}
}

// Concrete implementation
class UserEntity extends BaseEntity {
	private name: string;
	private email: string;

	constructor(id: string, name: string, email: string) {
		super(id);
		this.name = name;
		this.email = email;
	}

	validate(): boolean {
		return this.name.length > 0 && this.email.includes('@');
	}

	serialize(): object {
		return {
			id: this.id,
			name: this.name,
			email: this.email,
			createdAt: this.createdAt,
		};
	}

	getName(): string {
		return this.name;
	}

	getEmail(): string {
		return this.email;
	}
}

// Complex type definitions
type EventHandler<T> = (event: T) => void;
type EventMap = {
	'user:created': { userId: string; user: UserEntity };
	'user:updated': { userId: string; changes: Partial<UserEntity> };
	'user:deleted': { userId: string };
};

// Event emitter with type safety
class TypedEventEmitter {
	private listeners: Map<keyof EventMap, EventHandler<any>[]> = new Map();

	on<K extends keyof EventMap>(
		event: K,
		handler: EventHandler<EventMap[K]>
	): void {
		if (!this.listeners.has(event)) {
			this.listeners.set(event, []);
		}
		this.listeners.get(event)!.push(handler);
	}

	emit<K extends keyof EventMap>(event: K, data: EventMap[K]): void {
		const handlers = this.listeners.get(event) || [];
		handlers.forEach((handler) => handler(data));
	}

	off<K extends keyof EventMap>(
		event: K,
		handler: EventHandler<EventMap[K]>
	): void {
		const handlers = this.listeners.get(event) || [];
		const index = handlers.indexOf(handler);
		if (index > -1) {
			handlers.splice(index, 1);
		}
	}
}

// Conditional types
type NonNullable<T> = T extends null | undefined ? never : T;
type ReturnType<T> = T extends (...args: any[]) => infer R ? R : never;
type Parameters<T> = T extends (...args: infer P) => any ? P : never;

// Utility types
type Partial<T> = {
	[P in keyof T]?: T[P];
};

type Required<T> = {
	[P in keyof T]-?: T[P];
};

type Pick<T, K extends keyof T> = {
	[P in K]: T[P];
};

type Omit<T, K extends keyof T> = Pick<T, Exclude<keyof T, K>>;

// Mapped types
type Readonly<T> = {
	readonly [P in keyof T]: T[P];
};

type Record<K extends keyof any, T> = {
	[P in K]: T;
};

// Template literal types
type EventName<T extends string> = `on${Capitalize<T>}`;
type CSSProperty<T extends string> = `--${T}`;

// Complex function with overloads
function processValue(value: string): string;
function processValue(value: number): number;
function processValue(value: boolean): boolean;
function processValue(value: any): any {
	if (typeof value === 'string') {
		return value.toUpperCase();
	} else if (typeof value === 'number') {
		return value * 2;
	} else if (typeof value === 'boolean') {
		return !value;
	}
	return value;
}

// Namespace
namespace Utils {
	export function formatDate(date: Date): string {
		return date.toISOString();
	}

	export function parseDate(dateString: string): Date {
		return new Date(dateString);
	}

	export namespace Validation {
		export function isEmail(email: string): boolean {
			return email.includes('@');
		}

		export function isPhoneNumber(phone: string): boolean {
			return /^\d{10}$/.test(phone);
		}
	}
}

// Module augmentation
declare global {
	interface Array<T> {
		groupBy<K extends keyof T>(key: K): Record<string, T[]>;
	}
}

// Implementation of augmented method
Array.prototype.groupBy = function <T, K extends keyof T>(
	this: T[],
	key: K
): Record<string, T[]> {
	return this.reduce((groups, item) => {
		const groupKey = String(item[key]);
		if (!groups[groupKey]) {
			groups[groupKey] = [];
		}
		groups[groupKey].push(item);
		return groups;
	}, {} as Record<string, T[]>);
};

// Complex async/await patterns
async function processUsers(users: UserEntity[]): Promise<{
	valid: UserEntity[];
	invalid: UserEntity[];
	errors: string[];
}> {
	const valid: UserEntity[] = [];
	const invalid: UserEntity[] = [];
	const errors: string[] = [];

	for (const user of users) {
		try {
			if (user.validate()) {
				valid.push(user);
			} else {
				invalid.push(user);
				errors.push(`User ${user.getId()} failed validation`);
			}
		} catch (error) {
			errors.push(`Error processing user ${user.getId()}: ${error}`);
		}
	}

	return { valid, invalid, errors };
}

// Factory pattern with generics
class EntityFactory {
	static create<T extends BaseEntity>(
		entityClass: new (...args: any[]) => T,
		...args: any[]
	): T {
		return new entityClass(...args);
	}
}

// Usage examples
const userRepo: Repository<UserEntity, string> = {
	async findById(id: string) {
		// Mock implementation
		return new UserEntity(id, 'John Doe', 'john@example.com');
	},
	async save(entity: UserEntity) {
		// Mock implementation
		return entity;
	},
	async delete(id: string) {
		// Mock implementation
		return true;
	},
	async findAll() {
		// Mock implementation
		return [];
	},
};

const eventEmitter = new TypedEventEmitter();
eventEmitter.on('user:created', (event) => {
	console.log('User created:', event.userId);
});

// Export everything
export {
	MathService,
	UserEntity,
	TypedEventEmitter,
	processValue,
	processUsers,
	EntityFactory,
	userRepo,
	eventEmitter,
};

export namespace Utils {
	export const formatDate = Utils.formatDate;
	export const parseDate = Utils.parseDate;
	export namespace Validation {
		export const isEmail = Utils.Validation.isEmail;
		export const isPhoneNumber = Utils.Validation.isPhoneNumber;
	}
}

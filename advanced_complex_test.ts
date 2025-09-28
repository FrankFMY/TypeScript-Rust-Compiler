// Advanced TypeScript features test

// 1. Interfaces with generics
interface GenericInterface<T, U> {
	prop1: T;
	prop2: U;
	method1(): T;
	method2(param: U): void;
}

// 2. Union and intersection types
type StringOrNumber = string | number;
type UserWithId = { name: string } & { id: number };

// 3. Enum
enum Status {
	PENDING = 'pending',
	APPROVED = 'approved',
	REJECTED = 'rejected',
}

// 4. Class with inheritance
class BaseClass<T> {
	protected data: T;

	constructor(data: T) {
		this.data = data;
	}

	public getData(): T {
		return this.data;
	}
}

class DerivedClass extends BaseClass<string> {
	private status: Status;

	constructor(data: string, status: Status) {
		super(data);
		this.status = status;
	}

	public setStatus(status: Status): void {
		this.status = status;
	}

	public getStatus(): Status {
		return this.status;
	}

	// Getter and setter
	get value(): string {
		return this.data.toUpperCase();
	}

	set value(newValue: string) {
		this.data = newValue.toLowerCase();
	}

	// Static method
	static createDefault(): DerivedClass {
		return new DerivedClass('default', Status.PENDING);
	}
}

// 5. Function with generics
function processData<T>(data: T): T {
	return data;
}

// 6. Arrow function
const arrowFunction = (x: number): number => x * 2;

// 7. Object types
type ConfigObject = {
	readonly apiUrl: string;
	timeout?: number;
	retries: number;
};

// 8. Class with complex features
class ServiceClass {
	private config: ConfigObject;

	constructor(config: ConfigObject) {
		this.config = config;
	}

	public processRequest(): string {
		return `Processing with ${this.config.apiUrl}`;
	}

	private validateConfig(): boolean {
		return this.config.apiUrl.length > 0;
	}
}

// 9. Export statements
export { DerivedClass, BaseClass, Status, processData, arrowFunction };
export type { GenericInterface, StringOrNumber, UserWithId, ConfigObject };
export default ServiceClass;

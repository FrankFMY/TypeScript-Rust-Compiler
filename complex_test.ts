// Сложный TypeScript файл для полной проверки компилятора

// 1. Интерфейсы с дженериками
interface GenericInterface<T, U> {
	prop1: T;
	prop2: U;
	method1(): T;
	method2(param: U): void;
}

// 2. Типы с union и intersection
type StringOrNumber = string | number;
type UserWithId = { name: string } & { id: number };

// 3. Enum
enum Status {
	PENDING = 'pending',
	APPROVED = 'approved',
	REJECTED = 'rejected',
}

// 4. Класс с наследованием и дженериками
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

	// Getter и setter
	get value(): string {
		return this.data.toUpperCase();
	}

	set value(newValue: string) {
		this.data = newValue.toLowerCase();
	}

	// Static метод
	static createDefault(): DerivedClass {
		return new DerivedClass('default', Status.PENDING);
	}
}

// 5. Функция с дженериками и overloads
function processData<T>(data: T): T;
function processData<T>(data: T[]): T[];
function processData<T>(data: T | T[]): T | T[] {
	if (Array.isArray(data)) {
		return data.map((item) => item);
	}
	return data;
}

// 6. Arrow функции и async/await
const asyncFunction = async (): Promise<string> => {
	return new Promise((resolve) => {
		setTimeout(() => resolve('async result'), 100);
	});
};

// 7. Объектные типы с readonly и optional
type ConfigObject = {
	readonly apiUrl: string;
	timeout?: number;
	retries: number;
};

// 8. Mapped types
type PartialConfig<T> = {
	[P in keyof T]?: T[P];
};

// 9. Conditional types
type NonNullable<T> = T extends null | undefined ? never : T;

// 10. Template literal types
type EventName<T extends string> = `on${Capitalize<T>}`;

// 11. Класс с декораторами (симуляция)
class ServiceClass {
	private config: ConfigObject;

	constructor(config: ConfigObject) {
		this.config = config;
	}

	public async processRequest(): Promise<string> {
		const result = await asyncFunction();
		return `Processed: ${result}`;
	}

	private validateConfig(): boolean {
		return this.config.apiUrl.length > 0;
	}
}

// 12. Namespace
namespace Utils {
	export function formatString(str: string): string {
		return str.trim().toLowerCase();
	}

	export const VERSION = '1.0.0';
}

// 13. Module exports
export { DerivedClass, BaseClass, Status, processData, asyncFunction };
export type { GenericInterface, StringOrNumber, UserWithId, ConfigObject };
export default ServiceClass;

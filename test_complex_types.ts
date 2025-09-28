// Тест сложных типов
interface GenericInterface<T, U> {
	prop1: T;
	prop2: U;
	method1(): T;
	method2(param: U): void;
}

// Union и intersection типы
type StringOrNumber = string | number;
type UserWithId = { name: string } & { id: number };

// Класс с дженериками
class BaseClass<T> {
	protected data: T;

	constructor(data: T) {
		this.data = data;
	}

	public getData(): T {
		return this.data;
	}
}

// Enum
enum Status {
	PENDING = 'pending',
	APPROVED = 'approved',
	REJECTED = 'rejected',
}

export { GenericInterface, StringOrNumber, UserWithId, BaseClass, Status };

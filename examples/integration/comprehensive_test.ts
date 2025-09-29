// ULTRA MEGA TEST - Comprehensive TypeScript to Rust Compiler Test
// This test covers ALL possible TypeScript features and edge cases

// ===== BASIC TYPES =====
let primitiveString: string = 'Hello World';
let primitiveNumber: number = 42;
let primitiveBoolean: boolean = true;
let primitiveNull: null = null;
let primitiveUndefined: undefined = undefined;
let primitiveAny: any = 'anything';
let primitiveUnknown: unknown = 'unknown';
let primitiveNever: never = (() => {
	throw new Error('Never');
})();
let primitiveVoid: void = undefined;

// ===== ARRAYS =====
let stringArray: string[] = ['a', 'b', 'c'];
let numberArray: Array<number> = [1, 2, 3];
let mixedArray: (string | number)[] = ['a', 1, 'b', 2];
let nestedArray: number[][] = [
	[1, 2],
	[3, 4],
];
let readonlyArray: readonly string[] = ['readonly'];

// ===== OBJECTS =====
let simpleObject: { name: string; age: number } = { name: 'John', age: 30 };
let optionalObject: { name?: string; age?: number } = { name: 'Jane' };
let readonlyObject: { readonly name: string; readonly age: number } = {
	name: 'Bob',
	age: 25,
};
let indexSignature: { [key: string]: any } = { anything: 'goes' };
let mappedObject: { [K in 'a' | 'b']: string } = { a: 'A', b: 'B' };

// ===== INTERFACES =====
interface BasicInterface {
	id: number;
	name: string;
}

interface ExtendedInterface extends BasicInterface {
	email: string;
	isActive: boolean;
}

interface GenericInterface<T> {
	data: T;
	id: string;
}

interface OptionalInterface {
	required: string;
	optional?: number;
	readonly readonly: boolean;
}

interface MethodInterface {
	getName(): string;
	setName(name: string): void;
	getAge?(): number;
}

interface IndexInterface {
	[key: string]: any;
	[key: number]: string;
}

interface CallableInterface {
	(x: number, y: number): number;
	name: string;
}

interface ConstructableInterface {
	new (name: string): BasicInterface;
}

// ===== CLASSES =====
class BasicClass {
	public publicProp: string = 'public';
	private privateProp: number = 42;
	protected protectedProp: boolean = true;
	readonly readonlyProp: string = 'readonly';

	constructor(public paramProp: string) {}

	public publicMethod(): string {
		return this.publicProp;
	}

	private privateMethod(): number {
		return this.privateProp;
	}

	protected protectedMethod(): boolean {
		return this.protectedProp;
	}

	static staticMethod(): string {
		return 'static';
	}

	get getter(): string {
		return this.publicProp;
	}

	set setter(value: string) {
		this.publicProp = value;
	}
}

class ExtendedClass extends BasicClass {
	constructor(name: string, public newProp: number) {
		super(name);
	}

	override publicMethod(): string {
		return super.publicMethod() + ' extended';
	}
}

abstract class AbstractClass {
	abstract abstractMethod(): string;
	concreteMethod(): string {
		return 'concrete';
	}
}

class ConcreteClass extends AbstractClass {
	abstractMethod(): string {
		return 'implemented';
	}
}

class GenericClass<T> {
	constructor(public data: T) {}

	getData(): T {
		return this.data;
	}

	setData(data: T): void {
		this.data = data;
	}
}

// ===== ENUMS =====
enum BasicEnum {
	FIRST,
	SECOND,
	THIRD,
}

enum StringEnum {
	RED = 'red',
	GREEN = 'green',
	BLUE = 'blue',
}

enum MixedEnum {
	A = 1,
	B = 'b',
	C = 2,
}

const enum ConstEnum {
	X = 1,
	Y = 2,
}

// ===== FUNCTIONS =====
function basicFunction(x: number, y: number): number {
	return x + y;
}

function optionalParams(a: string, b?: number, c: string = 'default'): string {
	return `${a}${b || 0}${c}`;
}

function restParams(...args: number[]): number {
	return args.reduce((sum, arg) => sum + arg, 0);
}

function overloadedFunction(x: string): string;
function overloadedFunction(x: number): number;
function overloadedFunction(x: string | number): string | number {
	return x;
}

const arrowFunction = (x: number): number => x * 2;

// ===== GENERICS =====
function genericFunction<T>(arg: T): T {
	return arg;
}

function constrainedGeneric<T extends string>(arg: T): T {
	return arg;
}

function multipleGenerics<T, U>(first: T, second: U): [T, U] {
	return [first, second];
}

function defaultGeneric<T = string>(arg: T): T {
	return arg;
}

interface GenericConstraint<T extends { id: number }> {
	item: T;
}

// ===== UNION & INTERSECTION TYPES =====
type UnionType = string | number | boolean;
type IntersectionType = { name: string } & { age: number };
type LiteralUnion = 'red' | 'green' | 'blue';
type NumberLiteral = 1 | 2 | 3;
type BooleanLiteral = true | false;

// ===== CONDITIONAL TYPES =====
type IsString<T> = T extends string ? true : false;
type MyNonNullable<T> = T extends null | undefined ? never : T;
type Flatten<T> = T extends (infer U)[] ? U : T;
type MyReturnType<T> = T extends (...args: any[]) => infer R ? R : never;

// ===== MAPPED TYPES =====
type Partial<T> = {
	[P in keyof T]?: T[P];
};

type Required<T> = {
	[P in keyof T]-?: T[P];
};

type Readonly<T> = {
	readonly [P in keyof T]: T[P];
};

type Pick<T, K extends keyof T> = {
	[P in K]: T[P];
};

type Omit<T, K extends keyof T> = {
	[P in Exclude<keyof T, K>]: T[P];
};

// ===== TEMPLATE LITERAL TYPES =====
type EventName<T extends string> = `on${Capitalize<T>}`;
type UserEvent = EventName<'click' | 'hover' | 'focus'>;
type ApiEndpoint<T extends string> = `/api/${T}`;
type DatabaseKey<T extends string> = `db_${T}`;

// ===== UTILITY TYPES =====
type Record<K extends keyof any, T> = {
	[P in K]: T;
};

type Exclude<T, U> = T extends U ? never : T;
type Extract<T, U> = T extends U ? T : never;
type MyNonNullable2<T> = T extends null | undefined ? never : T;
type Parameters<T extends (...args: any) => any> = T extends (
	...args: infer P
) => any
	? P
	: never;
type ConstructorParameters<T extends new (...args: any) => any> =
	T extends new (...args: infer P) => any ? P : never;
type MyReturnType2<T extends (...args: any) => any> = T extends (
	...args: any
) => infer R
	? R
	: any;
type InstanceType<T extends new (...args: any) => any> = T extends new (
	...args: any
) => infer R
	? R
	: any;
type ThisParameterType<T> = T extends (this: infer U, ...args: any[]) => any
	? U
	: unknown;
type OmitThisParameter<T> = T extends (this: any, ...args: infer A) => infer R
	? (...args: A) => R
	: T;

// ===== DECLARATION MERGING =====
interface MergedInterface {
	prop1: string;
}

interface MergedInterface {
	prop2: number;
}

namespace MergedNamespace {
	export const value = 42;
}

namespace MergedNamespace {
	export const name = 'merged';
}

// ===== MODULES =====
export interface ExportedInterface {
	id: number;
}

export class ExportedClass {
	constructor(public name: string) {}
}

export function exportedFunction(): string {
	return 'exported';
}

export const exportedConstant = 'constant';

export default class DefaultExport {
	constructor(public value: string) {}
}

// ===== NAMESPACES =====
namespace MyNamespace {
	export interface Config {
		apiUrl: string;
		timeout: number;
	}

	export class Service {
		constructor(private config: Config) {}

		request(): string {
			return 'request';
		}
	}

	export namespace Utils {
		export function format(data: any): string {
			return JSON.stringify(data);
		}
	}
}

// ===== TEMPLATE LITERALS =====
const name = 'World';
const age = 25;
const templateLiteral = `Hello, ${name}! You are ${age} years old.`;
const multilineTemplate = `
    This is a multiline
    template literal with
    ${name} interpolation
`;

// ===== DESTRUCTURING =====
const array = [1, 2, 3, 4, 5];
const [first, second, ...rest] = array;
const [a, b, c = 'default'] = ['x', 'y'];

const obj = { x: 1, y: 2, z: 3 };
const { x, y, z } = obj;
const { x: renamedX, y: renamedY } = obj;
const { x: defaultX = 0 } = obj;

// ===== SPREAD OPERATOR =====
const spreadArray = [...array, 6, 7, 8];
const spreadObject = { ...obj, w: 4, v: 5 };

function spreadFunction(...args: number[]): number {
	return args.reduce((sum, arg) => sum + arg, 0);
}

// ===== COMPLEX EXPRESSIONS =====
const complexExpression = (a: number, b: number) => {
	if (a > b) {
		return a * 2;
	} else if (a < b) {
		return b * 2;
	} else {
		return a + b;
	}
};

const ternaryOperator = primitiveBoolean ? 'true' : 'false';
const nullishCoalescing = primitiveNull ?? 'default';
const optionalChaining = obj?.x?.toString();

// ===== TYPE ASSERTIONS =====
const typeAssertion = primitiveAny as string;
const angleBracketAssertion = <string>primitiveAny;
const constAssertion = 'literal' as const;

// ===== COMPLEX CONTROL FLOW =====
function complexControlFlow(value: string | number): string {
	switch (typeof value) {
		case 'string':
			return value.toUpperCase();
		case 'number':
			return value.toString();
		default:
			return 'unknown';
	}
}

// ===== ERROR HANDLING =====
class CustomError extends Error {
	constructor(message: string, public code: number) {
		super(message);
		this.name = 'CustomError';
	}
}

function errorHandling(): void {
	try {
		throw new CustomError('Something went wrong', 500);
	} catch (error) {
		if (error instanceof CustomError) {
			console.error(`Error ${error.code}: ${error.message}`);
		} else {
			console.error('Unknown error:', error);
		}
	} finally {
		console.log('Cleanup');
	}
}

// ===== ADVANCED PATTERNS =====
type Brand<T, B> = T & { __brand: B };
type UserId = Brand<number, 'UserId'>;
type ProductId = Brand<number, 'ProductId'>;

type DiscriminatedUnion =
	| { type: 'success'; data: string }
	| { type: 'error'; message: string }
	| { type: 'loading' };

function handleDiscriminatedUnion(union: DiscriminatedUnion): string {
	switch (union.type) {
		case 'success':
			return union.data;
		case 'error':
			return union.message;
		case 'loading':
			return 'Loading...';
	}
}

// ===== MAIN EXECUTION =====
function main(): void {
	console.log('Starting ULTRA MEGA TEST...');

	// Basic operations
	const result = basicFunction(5, 3);
	console.log('Basic function result:', result);

	// Class instantiation
	const basicInstance = new BasicClass('test');
	const extendedInstance = new ExtendedClass('extended', 42);

	// Generic usage
	const genericInstance = new GenericClass<string>('generic data');
	const genericResult = genericFunction<number>(42);

	// Template literals
	console.log('Template literal:', templateLiteral);

	// Complex expressions
	const complexResult = complexExpression(10, 5);
	console.log('Complex expression result:', complexResult);

	// Error handling
	errorHandling();

	// Discriminated union
	const success: DiscriminatedUnion = { type: 'success', data: 'Success!' };
	const error: DiscriminatedUnion = {
		type: 'error',
		message: 'Error occurred',
	};
	const loading: DiscriminatedUnion = { type: 'loading' };

	console.log('Discriminated union results:');
	console.log(handleDiscriminatedUnion(success));
	console.log(handleDiscriminatedUnion(error));
	console.log(handleDiscriminatedUnion(loading));

	console.log('ULTRA MEGA TEST completed successfully!');
}

// Export everything for testing
export {
	BasicClass,
	ExtendedClass,
	AbstractClass,
	ConcreteClass,
	GenericClass,
	basicFunction,
	genericFunction,
	main,
	CustomError,
	MyNamespace,
};

export type {
	BasicInterface,
	ExtendedInterface,
	GenericInterface,
	UnionType,
	IntersectionType,
	DiscriminatedUnion,
	UserId,
	ProductId,
};

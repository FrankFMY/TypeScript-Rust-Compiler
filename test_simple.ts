class SimpleClass {
	public prop1: string = 'hello';

	constructor() {
		this.prop1 = 'world';
	}

	public method1(): string {
		return this.prop1;
	}
}

export { SimpleClass };

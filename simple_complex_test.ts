// Простой сложный тест
interface TestInterface {
	prop: string;
}

class TestClass {
	private prop: string;

	constructor(prop: string) {
		this.prop = prop;
	}

	public getProp(): string {
		return this.prop;
	}
}

export { TestClass };

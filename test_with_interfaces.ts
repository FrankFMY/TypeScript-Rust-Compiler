interface CallableInterface {
	(x: number, y: number): number;
	name: string;
}

interface ConstructableInterface {
	new (name: string): BasicInterface;
}

class BasicClass {
	public publicProp: string = 'public';
	private privateProp: number = 42;
	protected protectedProp: boolean = true;
	readonly readonlyProp: string = 'readonly';

	constructor(public paramProp: string) {}

	public publicMethod(): string {
		return this.publicProp;
	}
}

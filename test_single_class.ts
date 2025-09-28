class BasicClass {
	public publicProp: string = 'public';
	private privateProp: string = 'private';
	protected protectedProp: string = 'protected';
	readonly readonlyProp: string = 'readonly';

	constructor() {
		this.publicProp = 'initialized';
	}

	public publicMethod(): string {
		return this.publicProp;
	}

	private privateMethod(): string {
		return this.privateProp;
	}

	protected protectedMethod(): boolean {
		return true;
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

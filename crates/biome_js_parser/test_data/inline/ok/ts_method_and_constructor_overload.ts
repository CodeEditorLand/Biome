class Test {
	constructor();
	constructor(a: string); // ASI
	constructor(a?: string) {}
	async method(): Promise<string>;
	method(a: string): Promise<string>; // ASI
	async method(a?: string): Promise<string> {
		return "test";
	}
}

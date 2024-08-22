class Foo {
	@dec constructor() {}
	@dec [index: string]: { props: string }
}
class Quiz {
	@dec public constructor() {}
}
class Bar extends Foo {
	@dec
   constructor();
	constructor(a: string);
	constructor(a?: string) {}
}
declare class Baz {
	@dec method();
	@dec get foo();
	@dec set foo(a);
}

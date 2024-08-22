interface A {
	a: string;
}
const x = { a: "test" } satisfies A;
const y = { a: "test", b: "test" } satisfies A;
const z = undefined satisfies 1;
let not_a_satisfies_expression;
satisfies;
const precedence = (("hello" satisfies string) +
	3) satisfies number satisfies number;

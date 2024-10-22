/* should not generate diagnostics */
declare class Test {
	constructor();
	get prop();
	set prop(value: any);
	method(): void;
	arrow: () => string;
	private method(): void;
}

declare function test(): void;

declare var fn: () => number;

declare var arrowFn: () => string;

declare const obj: {
	method(): string;
};

declare const obj: {
	readonly method: string;
};

declare const _default: () => void;
export default _default;

declare const func: (value: number) => {
	readonly foo: "bar";
	readonly value: number;
};
declare const func: () => { readonly x: typeof x };

declare const node: any;
declare const arr: any[];
declare const foo: number[];

declare function fn(callback: () => void): void;

declare const IIFE1: void;
declare const IIFE2: void;

declare function test(a: number, b: number): void;
declare function test(): void;

declare var fn: () => number;

declare var arrowFn: () => string;

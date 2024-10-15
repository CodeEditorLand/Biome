/* should not generate diagnostics */
function test(): void {
  return;
}

var fn = function (): number {
  return 1;
};

var arrowFn = (): string => 'test';

class Test {
  constructor() {}
  get prop(): number {
    return 1;
  }
  set prop() {}
  method(): void {
    return;
  }
  arrow = (): string => 'arrow';
}

const obj = {
	method(): string {
		return "test"
	}
}

const obj = {
  get method(): string {
    return "test"
  },
};

export default (): void => {};
export default function (): void {}

// check direct const assertions
const func = (value: number) => ({ foo: 'bar', value }) as const;
const func = () => x as const;


// check allow expressions
node.addEventListener('click', () => {});
node.addEventListener('click', function () {});
const foo = arr.map(i => i * i);
fn(() => {});
fn(function () {});
new Promise(resolve => {});
new Foo(1, () => {});
[function () {}, () => {}];
(function () {
  console.log("This is an IIFE");
})();
(() => {
  console.log("This is an IIFE");
})();
setTimeout(function() { console.log("Hello!"); }, 1000);


// check higher order functions
const arrowFn = () => (): void => {};
const arrowFn = () => function(): void {}
const arrowFn = () => {
  return (): void => { };
}


// type assertion
const asTyped = (() => '') as () => string;
const castTyped = <() => string>(() => '');

// variable declarator with a type annotation 
type FuncType = () => string;
const arrowFn: FuncType = () => 'test';
const funcExpr: FuncType = function () {
  return 'test';
};

// default parameter with a type annotation
type CallBack = () => void;
const f = (gotcha: CallBack = () => { }): void => { };
function f(gotcha: CallBack = () => {}): void {}

// class property with a type annotation
type MethodType = () => void;
class App {
    private method: MethodType = () => { };
}

// function as a property or a nested property of a typed object
const x: Foo = { prop: () => {} }
const x = { prop: () => {} } as Foo
const x = <Foo>{ prop: () => {} }

const x: Foo = { bar: { prop: () => {} } }

class Accumulator {
  private count: number = 0;
  public accumulate(fn: () => number): void {
      this.count += fn();
  }
}
new Accumulator().accumulate(() => 1);
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
type Foo<in T> = {};
type Foo<out> = {};
type Foo<out T> = {};
type Foo<in out> = {};
type Foo<out out> = {};
type Foo<in out out> = {};
type Foo<in X, out Y> = {};
type Foo<out X, in Y> = {};
type Foo<out X, out Y extends keyof X> = {};
class Foo<in T> {}
class Foo<out T> {}
export default class Foo<in T> {}
class Foo<out T> {}
type Foo<in T> = {};
type Foo<out T> = {};
declare class Foo<in T> {}
declare class Foo<out T> {}
declare type Foo<in T> = {};
declare type Foo<out T> = {};
function a<const T>() {}
function b<const T extends U>() {}
function c<T, const U>() {}
declare function d<const T>();
<T>() => {};
<const T>() => {};
<const T>() => {};
<const T extends U>() => {};
<T, const U>() => {};
class A<const T> {}
class B<const T extends U> {}
class C<T, const U> {}
class D<const in T> {}
class E<const in T> {}
class F<const in out T> {}
(class<const T> {});
(class<const T extends U> {});
(class<T, const U> {});
(class<const in T> {});
(class<const in T> {});
class _ {
	method<const T>() {}
	method<const T extends U>() {}
	method<T, const U>() {}
}
declare namespace a {
	function test<const T>(): T;
}
const obj = {
	a<const T>(b: any): b is T {
		return true;
	},
};

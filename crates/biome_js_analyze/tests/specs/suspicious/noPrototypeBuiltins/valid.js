Object.hasOwn(foo, "bar");
Object.prototype.hasOwnProperty.call(foo, "bar");
Object.prototype.isPrototypeOf.call(foo, bar);
Object.prototype.propertyIsEnumerable.call(foo, 'bar');
Object.prototype.hasOwnProperty.apply(foo, ['bar']);
Object.prototype.isPrototypeOf.apply(foo, ['bar']);
Object.prototype.propertyIsEnumerable.apply(foo, ['bar']);
foo.hasOwnProperty;
foo.hasOwnProperty.bar();
foo(hasOwnProperty);
hasOwnProperty(foo, 'bar');
isPrototypeOf(foo, 'bar');
propertyIsEnumerable(foo, 'bar');
({}.hasOwnProperty.call(foo, 'bar'));
({}.isPrototypeOf.call(foo, 'bar'));
({}.propertyIsEnumerable.call(foo, 'bar'));
({}.hasOwnProperty.apply(foo, ['bar']));
({}.isPrototypeOf.apply(foo, ['bar']));
({}.propertyIsEnumerable.apply(foo, ['bar']));
foo[hasOwnProperty]('bar');
foo['HasOwnProperty']('bar');
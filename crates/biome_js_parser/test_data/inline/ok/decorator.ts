// class expressions
const a =
	@decorator
	class {};
const b =
	@decorator
	@functionDecorator(1, 2, 3)
	class {};
const c =
	@first
	@second
	class Foo {};
// class declarations
@decorator
class Foo {}
@decorator
@functionDecorator(1, 2, 3)
class Bar {}
@first
@second
class Baz {}
// abstract class declarations
@decorator
abstract class Foo {}
@decorator
@functionDecorator(1, 2, 3)
abstract class Bar {}
@first
@second
abstract class Baz {}
// exported class declarations
export
@decorator
class Foo {}
export
@decorator
@functionDecorator(1, 2, 3)
class Bar {}
export
@first
@second
class Baz {}
@decorator
export class Foo {}
@first.field
@second
@(() => decorator)()
export class Bar {}
@before
@after
export class Foo {}
@before.field
@before
@(() => decorator)()
@after.field
@after
@(() => decorator)()
export class Bar {}

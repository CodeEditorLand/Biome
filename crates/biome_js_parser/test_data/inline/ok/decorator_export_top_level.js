@decorator
export class Foo {}
@first.field
@second
@(() => decorator)()
export class Bar {}
@before
@after
export class Foo {}
@before
export abstract class Foo {}
@before
@after
export abstract class Foo {}

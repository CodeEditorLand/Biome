const a = <A, B extends A, C = string>(a: A, b: B, c: C) => "hello";
const b = async <A, B>(a: A, b: B): Promise<string> => "hello";

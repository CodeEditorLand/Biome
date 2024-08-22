const x: any = "string";
const y = x as string;
const z = x as const;
const not_an_as_expression = x;
as;
const precedence = (("hello" as const) + 3) as number as number;

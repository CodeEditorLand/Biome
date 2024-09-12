export enum Status {
	Open = 1,
	Close = 2,
}

export enum Color {
	Red = "Red",
	Green = "Green",
	Blue = "Blue",
}

export enum Mix {
	Str = "Str",
	Number = 0,
}

export declare enum Weather {
    Rainy,
    Sunny,
}

declare enum Weather2 {
    Rainy,
    Sunny,
}

export declare namespace A {
    export namespace B {
        export enum Enum {
            A,
            B,
        }
    }
}

declare namespace A2 {
    export namespace B2 {
        export enum Enum {
            A,
            B,
        }
    }
}

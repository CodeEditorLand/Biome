const a = "b";
const [c, b] = [1, 2];
const [d, ...abcd] = [1];
const [e = "default", x] = [];
const [, f, ...rest] = [];
const [[...rest2], { g }] = [];

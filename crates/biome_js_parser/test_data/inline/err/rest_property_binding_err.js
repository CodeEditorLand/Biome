const { ... } = a;
let { ...c = "default" } = a;
const { ...{a} } = b;
let { ...rest, other_assignment } = a;
let { ...rest2, } = a;
async function test() {
	const { ...await } = a;
}

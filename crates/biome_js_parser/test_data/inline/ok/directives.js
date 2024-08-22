// SCRIPT
"use new";
const a = 10;
("use strict"); // not a directive
function test() {
	const b = 10;
	("use strict"); // not a directive
}
() => {
	"use strict".length; // not a directive
	const c = 10;
	("use strict"); // not a directive
};
const b = () => {
	const e = 10;
	("use strict"); // not a directive
};
{
	("use strict"); // not a directive
}

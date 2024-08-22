enum A {}
enum B {
	a = 0,
	b = 1,
	c = 2,
}
enum C {
	A = 1,
	B = A * 2,
	["A"] = 3,
}

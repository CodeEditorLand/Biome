while (foo) {
 bar();
}

if (foo) {
 if (bar) {
  baz();
 }
}

function bar() {
 baz();
}

{
 let x = 1;
}

{
 const y = 1;
}

{
 class Foo {}
}

aLabel: {
}

class C {
 static {
  lbl: {
   if (something) {
    break lbl;
   }

   foo();
  }
 }
}

if (x) {}

function g() {
 let i;
 for(i = 0; f(i); i++) {}
 return i;
}

function JsTryStatement() {
    try {
     foo();
    } catch (error) {
     bar();
    } finally {
     baz();
    }
   }
   

function JsTryFinallyStatement() {
 try {
  foo();
 } catch (error) {
  bar();
 } finally {
  baz();
 }
}

switch (1) {
  default: {
    console.info("1");
    console.info("2");
    break;
  }
}

switch (foo) {
  case true: {
    console.info("1");
    console.info("2");
    break;
  }
}

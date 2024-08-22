// SCRIPT
function f() {
    const a = <div>a</div>; // JSX
    const b = <string>b; // type assertion
    let c = <string>b<a>d; // type assertion
    let d = <div>a</div>/; // ambiguous: JSX or "type assertion a less than regex /div>/". Probably JSX.
    let d = <string>a</string>/;
}

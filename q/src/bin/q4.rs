// Q4 (intermediate): `store(&s)` fails at the call site but `store("world")`
// works. What does the `'static` lifetime bound actually mean? Why does a string
// literal satisfy `&'static str` while a reference to a locally-owned `String`
// does not? (Follow-up: is `'static` a property of the reference or of the data
// it points to?)

fn store(_s: &'static str) {
    // pretend we stash it in a global
}

fn main() {
    let s = String::from("hello");
    store(&s);      // fails
    store("world"); // works
}

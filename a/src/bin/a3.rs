// Q3 (intermediate): Why won't `dangle` compile? Define what a "dangling
// reference" is and explain how the borrow checker rules it out here. Then give
// the idiomatic fix (hint: you probably don't want to return a reference at all).

fn no_dangle() -> String {
    String::from("hello")
}

fn main() {
    let s = no_dangle();
    println!("{}", s);
}

// ANSWER:
//
// A "dangling reference" is a reference whose referent has already been freed
// — reading it would be undefined behavior. C lets you create them trivially;
// Rust's borrow checker makes them impossible in safe code.
//
// The original `dangle` creates a `String` owned by the function's stack
// frame, then tries to return a reference to it. When `dangle` returns, `s` is
// dropped, so the returned `&String` would point at freed memory. The
// compiler's analysis is simple: the return type `&String` requires some
// lifetime `'a` tied to a caller-visible scope, but `s` only lives inside
// `dangle`, so no valid `'a` exists. You'll see E0106 ("missing lifetime
// specifier") followed by the suggestion to return an owned `String`.
//
// The idiomatic fix is to transfer ownership to the caller by returning
// `String` directly (as `no_dangle` above). No borrow is involved, so no
// lifetime is needed.

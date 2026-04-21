// Q0 (beginner): The following function fails to compile. Why does Rust need
// a lifetime annotation here, and what is the minimal annotation that makes it
// compile? What exactly does that annotation promise the caller?

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn main() {
    let a = String::from("hello");
    let b = String::from("world!");
    println!("{}", longest(&a, &b));
}

// ANSWER:
//
// Every reference in Rust has a lifetime. Usually the compiler can infer them
// via elision, but elision only fills in output lifetimes when (a) there is
// exactly one input reference, or (b) one of the inputs is `&self` / `&mut
// self`. Here there are TWO input references and no `self`, so the compiler
// cannot decide which input's lifetime the returned `&str` should borrow from,
// and it refuses to guess.
//
// The fix is to introduce a named lifetime `'a` shared by both inputs and the
// output. That signature promises: "the returned reference is valid for the
// intersection of the lifetimes of `x` and `y`" — i.e. for as long as BOTH
// inputs are alive. That's sound because the function may return either one.

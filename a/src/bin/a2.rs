// Q2 (beginner -> intermediate): The following struct fails to compile. Why
// does Rust require a lifetime parameter on any struct that holds a reference?
// What invariant is the compiler enforcing about `Excerpt` and the data `part`
// refers to?

struct Excerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let excerpt = Excerpt { part: first_sentence };
    println!("{}", excerpt.part);
}

// ANSWER:
//
// A struct that stores a reference is only meaningful while the referent is
// still alive. The lifetime parameter `<'a>` ties the struct's own existence
// to the lifetime of whatever `part` points at: the compiler enforces
// "`Excerpt<'a>` cannot outlive `'a`".
//
// Without `<'a>`, the compiler has no way to express that constraint, so it
// rejects the definition outright (E0106: "missing lifetime specifier"). Once
// annotated, the borrow checker will refuse to drop `novel` while any
// `Excerpt<'_>` borrowing from it is still in scope — exactly the dangling-
// reference protection we want.
//
// Note: with `struct Excerpt<'a> { part: &'a str }` you never have to annotate
// at the use site — `Excerpt { part: first_sentence }` infers `'a`.

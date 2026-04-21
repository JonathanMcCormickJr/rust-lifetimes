// Q2 (beginner -> intermediate): The following struct fails to compile. Why
// does Rust require a lifetime parameter on any struct that holds a reference?
// What invariant is the compiler enforcing about `Excerpt` and the data `part`
// refers to?

struct Excerpt {
    part: &str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let excerpt = Excerpt { part: first_sentence };
    println!("{}", excerpt.part);
}

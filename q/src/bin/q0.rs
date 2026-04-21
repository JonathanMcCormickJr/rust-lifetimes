// Q0 (beginner): The following function fails to compile. Why does Rust need
// a lifetime annotation here, and what is the minimal annotation that makes it
// compile? What exactly does that annotation promise the caller?

fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() { x } else { y }
}

fn main() {
    let a = String::from("hello");
    let b = String::from("world!");
    println!("{}", longest(&a, &b));
}

// Q3 (intermediate): Why won't `dangle` compile? Define what a "dangling
// reference" is and explain how the borrow checker rules it out here. Then give
// the idiomatic fix (hint: you probably don't want to return a reference at all).

fn dangle() -> &String {
    let s = String::from("hello");
    &s
}

fn main() {
    let s = dangle();
    println!("{}", s);
}

// Q6 (advanced): Why does the bound use `for<'a> Fn(&'a str) -> &'a str`
// instead of a single named lifetime such as `Fn(&'b str) -> &'b str` tied to
// an outer parameter? In plain words, what does `for<'a>` (a higher-ranked
// trait bound) tell the compiler about the closure? Construct a call site that
// would be rejected WITHOUT the HRTB but accepted WITH it.

fn apply<F>(f: F) -> String
where
    F: for<'a> Fn(&'a str) -> &'a str,
{
    let owned = String::from("  hello world  ");
    let borrowed: &str = f(&owned);
    borrowed.to_string()
}

fn trim_ws(s: &str) -> &str {
    s.trim()
}

fn main() {
    println!("{:?}", apply(trim_ws));
}

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

// ANSWER:
//
// `for<'a> Fn(&'a str) -> &'a str` is a higher-ranked trait bound. In English:
// "`F` implements `Fn(&str) -> &str` for EVERY possible lifetime `'a`." It's
// universal quantification over lifetimes, baked into the trait bound itself.
//
// Contrast with `fn apply<'b, F>(f: F) -> String where F: Fn(&'b str) -> &'b
// str`. There `'b` is a single lifetime chosen by the CALLER at each call to
// `apply`. Inside `apply` we create a local `String` and borrow from it — that
// borrow has some lifetime `'local` that only exists inside the function body
// and cannot be named from outside. There is no caller-provided `'b` that
// matches `'local`, so the non-HRTB version fails to type-check when we try
// to call `f(&owned)`.
//
// The HRTB version succeeds because `F` is required to work for EVERY
// lifetime, so in particular it works for `'local`. `trim_ws` (and any plain
// `fn(&str) -> &str` with elided lifetimes) satisfies the HRTB automatically
// because its signature is itself generic over the input lifetime.
//
// Rejected without HRTB / accepted with HRTB: the very call inside `apply` —
// `f(&owned)` where `owned: String` is function-local. More broadly, any
// higher-order combinator that passes short-lived borrows into a caller-
// supplied closure (e.g. `Iterator::for_each`-style callbacks on borrowed
// elements) relies on HRTBs.
//
// Aside: when you write `Fn(&str) -> &str` with no explicit lifetimes in a
// bound position, Rust desugars it AS an HRTB — `for<'a> Fn(&'a str) -> &'a
// str` — so most code gets this for free.

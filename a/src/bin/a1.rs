// Q1 (beginner): This function compiles with no explicit lifetime annotations.
// Name the lifetime elision rule that makes that possible. Then: if the signature
// took TWO `&str` parameters instead of one, would elision still apply? Why or why not?

fn first_word(s: &str) -> &str {
    s.split_whitespace().next().unwrap_or("")
}

fn main() {
    let s = String::from("hello world");
    println!("{}", first_word(&s));
}

// ANSWER:
//
// Rust applies three lifetime elision rules to `fn` signatures:
//   1. Each elided input reference gets its own distinct lifetime parameter.
//   2. If there is exactly one input lifetime, it is assigned to every elided
//      output lifetime.
//   3. If one of the inputs is `&self` or `&mut self`, that lifetime is
//      assigned to every elided output lifetime.
//
// `first_word(s: &str) -> &str` desugars via rules 1 and 2 to
// `fn first_word<'a>(s: &'a str) -> &'a str` — exactly one input lifetime, so
// the output borrows from it.
//
// With TWO `&str` inputs and no `&self`, rule 2 no longer applies and rule 3
// never did, so elision leaves the output lifetime undetermined and the
// compiler rejects the signature. You must annotate explicitly — see Q0.

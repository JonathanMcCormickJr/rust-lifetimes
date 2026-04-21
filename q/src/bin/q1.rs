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

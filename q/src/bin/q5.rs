// Q5 (intermediate -> advanced): `pick_first` compiles, but the `main` below
// fails to compile. The return is only ever derived from `x`, yet the caller is
// punished because `'a` gets unified with the *shorter* of the two input
// lifetimes. Rewrite the signature using TWO lifetime parameters so that the
// returned reference is tied only to `x`, and explain why one lifetime was
// overly restrictive.

fn pick_first<'a>(x: &'a str, y: &'a str) -> &'a str {
    let _ = y;
    x
}

fn main() {
    let long = String::from("long-lived");
    let result;
    {
        let short = String::from("short");
        result = pick_first(&long, &short);
    }
    println!("{}", result);
}

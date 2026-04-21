// Q5 (intermediate -> advanced): `pick_first` compiles, but the `main` below
// fails to compile. The return is only ever derived from `x`, yet the caller is
// punished because `'a` gets unified with the *shorter* of the two input
// lifetimes. Rewrite the signature using TWO lifetime parameters so that the
// returned reference is tied only to `x`, and explain why one lifetime was
// overly restrictive.

fn pick_first<'a, 'b>(x: &'a str, _y: &'b str) -> &'a str {
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

// ANSWER:
//
// In the single-lifetime version `fn pick_first<'a>(x: &'a str, y: &'a str) ->
// &'a str`, the compiler picks a single `'a` that BOTH arguments must satisfy.
// When the call site supplies a long-lived `&long` and a short-lived `&short`,
// the only lifetime that fits both is the shorter one — so the returned
// reference inherits the shorter lifetime too, even though it was never
// actually derived from `y`. That forces `result` to be dropped at end of the
// inner block, and the `println!` fails.
//
// Splitting into two lifetimes `<'a, 'b>` says "`x` lives for some `'a`, `y`
// lives for some possibly different `'b`, and the return borrows only from
// `x`." Now `'b` is free to be short while `'a` stays long, so
// `result: &'a str` remains valid beyond the inner block.
//
// Rule of thumb: give each reference its own lifetime parameter, and only
// unify them when there's a real constraint (e.g. the output might be derived
// from either input, as in the classic `longest`). Overconstraining lifetimes
// is a common source of mysterious borrow-checker errors at the CALLER even
// when the function body looks fine.

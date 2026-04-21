// Q4 (intermediate): `store(&s)` fails at the call site but `store("world")`
// works. What does the `'static` lifetime bound actually mean? Why does a string
// literal satisfy `&'static str` while a reference to a locally-owned `String`
// does not? (Follow-up: is `'static` a property of the reference or of the data
// it points to?)

fn store(_s: &'static str) {
    // pretend we stash it in a global
}

fn main() {
    // `"world"` is a string literal baked into the binary's read-only data
    // segment, so it is valid for the entire program — i.e. `&'static str`.
    store("world");

    // A `String` owned by `main` lives only until `main` returns, so `&s` is
    // `&'a str` for some `'a` bounded by `main`'s scope. It does NOT satisfy
    // `'static`, and `store(&s)` would be rejected.
    let s = String::from("hello");
    let _local_ref: &str = &s;
    let _ = s;
}

// ANSWER:
//
// `'static` is the longest possible lifetime: the referent is valid for the
// entire duration of the program. On a reference bound, `&'static T` means
// "this reference may be held indefinitely because the data it points to will
// never be dropped." Typical sources: string literals (`&'static str` in
// `.rodata`), items declared `static`, and values deliberately leaked
// (`Box::leak`, `Vec::leak`, `OnceLock::get_or_init`, etc).
//
// A local `String` like `s` is owned by its enclosing function. At end of
// scope it's dropped, so any `&str` borrowed from it has a lifetime bounded by
// that scope — strictly shorter than `'static`. Passing `&s` where `&'static
// str` is required is therefore rejected (E0597 / E0521 variants depending on
// call shape). The literal `"world"` works because its bytes are stored in the
// binary itself and outlive every scope.
//
// Follow-up: `'static` is a property of the REFERENT. A reference simply
// carries the lifetime of the data it points to. Saying "the reference is
// 'static" is shorthand for "the data it borrows lives forever, so the
// reference may be held forever."
//
// Aside: the trait bound `T: 'static` is related but distinct — it means
// "values of `T` contain no non-'static borrows," i.e. `T` can be held
// indefinitely. An owned `String` satisfies `String: 'static` even though
// `&String` from a local does not satisfy `&'static String`.

// Q7 (expert): This program fails to compile. Explain the failure in terms of
// VARIANCE. Specifically:
//   - Why is `&'a T` covariant in `T` but `&'a mut T` invariant in `T`?
//   - Walk through the concrete unsoundness that invariance on `&mut T`
//     prevents by showing what would happen if the compiler accepted
//     `assign_shorter(&mut target, &local)`.
// (Bonus: name one stdlib type that is invariant for the same reason.)

// The function itself is fine; the unsoundness lives at the CALL SITE, which
// is why we leave it commented out. Uncomment to observe the E0597 error.
fn assign_shorter<'short>(dst: &mut &'short str, src: &'short str) {
    *dst = src;
}

fn main() {
    let mut target: &'static str = "initial";
    {
        let local = String::from("local");
        // assign_shorter(&mut target, &local); // rejected: `local` does not live long enough
        let _ = &local;
    }
    assign_shorter(&mut target, "still-static");
    println!("{}", target);
}

// ANSWER:
//
// Variance describes how subtyping on a type parameter relates to subtyping on
// the whole type. Rust's only subtyping is lifetime outliving: `'long: 'short`
// means `&'long T` is a subtype of `&'short T` (a longer lifetime can stand in
// where a shorter one is required).
//
// `&'a T` is COVARIANT in `T`: you can freely shorten the lifetime of the
// referent because you can only READ through the reference. Handing out a
// reference to data that lives LONGER than advertised is never unsound.
//
// `&'a mut T` is INVARIANT in `T`: you can both read AND write. Writing a
// value of type `U` into a location typed `T` requires `U` to be usable WHERE
// `T` is required, i.e. `U` to be a SUBTYPE of `T` — that's CONTRAvariant.
// Reading back out wants COvariance. Needing both at once collapses to
// invariance: `T` must be EXACTLY `T`, not a sub- or super-type.
//
// Unsoundness prevented:
//   let mut target: &'static str = "initial";
//   {
//       let local = String::from("local");
//       assign_shorter(&mut target, &local); // if this compiled...
//   } // `local` is dropped here
//   println!("{}", target);                   // ...`target` would now read
//                                             //    freed memory: UB.
//
// If `&mut T` were covariant in `T`, the compiler would coerce `&mut
// &'static str` down to `&mut &'short str` at the call, letting
// `assign_shorter` stash a short-lived `&str` into a slot the outer code
// still thinks holds `&'static str`. Invariance blocks the coercion and
// rejects the call at compile time (E0597: "`local` does not live long
// enough").
//
// Bonus invariant types in the stdlib: `Cell<T>`, `RefCell<T>`, `UnsafeCell<T>`,
// `*mut T`, and `PhantomData<fn(T) -> T>` are all invariant in `T` for the
// same "read and write through the same handle" reason.

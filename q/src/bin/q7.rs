// Q7 (expert): This program fails to compile. Explain the failure in terms of
// VARIANCE. Specifically:
//   - Why is `&'a T` covariant in `T` but `&'a mut T` invariant in `T`?
//   - Walk through the concrete unsoundness that invariance on `&mut T`
//     prevents by showing what would happen if the compiler accepted
//     `assign_shorter(&mut target, &local)`.
// (Bonus: name one stdlib type that is invariant for the same reason.)

fn assign_shorter<'short>(dst: &mut &'static str, src: &'short str) {
    *dst = src;
}

fn main() {
    let mut target: &'static str = "initial";
    {
        let local = String::from("local");
        assign_shorter(&mut target, &local);
    }
    // If the call above compiled, `target` would now reference freed memory.
    println!("{}", target);
}

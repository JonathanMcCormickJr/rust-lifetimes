# lifetimes

A set of eight Rust exercises on lifetimes, borrowing, and variance, ranging from beginner to expert.

## Layout

This is a Cargo workspace with two crates:

- `q/` — the questions. Each `q/src/bin/qN.rs` poses a problem (usually code that fails to compile) and asks you to explain and fix it.
- `a/` — the answers. Each `a/src/bin/aN.rs` contains the working code plus a written explanation.

Exercises are numbered `0` through `7`, roughly in order of increasing difficulty.

## Usage

Try each question on your own first:

```sh
cargo run -p q --bin q0
```

Then compare against the answer:

```sh
cargo run -p a --bin a0
```

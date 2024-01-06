# RUST

Exploring Rust.

## commands

```bash
cargo new explore            # create project
cargo test                   # run tests
cargo clippy                 # format
cargo build                  # binary build
./target/debug/exploring     # binary run

```

## rust

Evaluated at compile time. Compiler picks up afterwards.
Avoid code duplication, variadic function support (functions that can take variable number of arguments).

memory management model: ownership (rather than GC)

Exceptions Vs Monoadic error handling. result and option types.

Rust mostly functional although 
railroad oriented programming (result types on happy or error path)

chaining style -> optional value


by default `variables` have constant behaviour

### error handling

Informally known as Railway Oriented Programming, or Guard Rails. Metaphor
of switching between success and error tracks.

In contrast with Exception handling where errors 'bubble' up. This can lead to 
uncaught Exceptions.

The `Option` and `Result` types are used for explicit error handling. Aligns with
goals of safety and explicitness.

Integrates error handling into normal control flow which may make it easier to
reason about.

#### Option

Enum with two variants: `Some(T)` and `None`.

Use when value may be absent. Used where null or absent values used in other langs.

Handling of absence, thus avoiding common null reference errors.

#### Result

Enum with two variants: `Ok(T)` and `Err(E)`

Use for error handling. Functions that may fail return a `Result` (instead of raising Exception).

Caller must explicitly handle or propagate the failure.

## srcs

https://www.youtube.com/watch?v=MoqtsYLGCC4&ab_channel=ArjanCodes

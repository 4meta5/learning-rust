# learning rust

Best practices are haphazardly collected in [best](./best).

## Getting Started With Rust

[`practice/`](./practice) contains the following examples:
* [`linkedlist.rs`](./practice/simple/src/linkedlist.rs): very simple linked list implementation to demonstrate the `struct`-`impl` code pattern common in Rust (from [abeinges too many lists book](http://cglab.ca/~abeinges/blah/too-many-lists/book/))
* [`recurrence.rs`](./practice/simple/src/recurrence.rs): macros in Rust with recursion via the fibonacci sequence
* [`conversion.rs`](./practice/simple/src/conversion.rs): practicing `From` and `Into` for conversion between types
* [`statemachine.rs`](./practice/simple/src/statemachine.rs): Rust state machine pattern by [Hoverbear](https://hoverbear.org/2016/10/12/rust-state-machine-pattern/)
* [`combinators.rs`](./practice/simple/src/combinator.rs): basic syntax and patterns for combinators (`map`, `and_then`)
* [`container.rs`](./practice/simple/src/container.rs): `Box<T>` vs `Rc<T>` in the context of Trait dynamic dispatch
* [`practice/serialization/`](./practice/serialization) defines a [basic JSON serializer with serde](./practice/serialization/serializer/src/lib.rs)
* **[generic code patterns](./practice/README.md#pattern)**

**More Resources**
* [The Rust Book](https://doc.rust-lang.org/book/index.html)
* [Rust by Example](https://doc.rust-lang.org/rust-by-example/index.html)
* [cheats.rs](https://cheats.rs)
* [brson/rust-anthology](https://github.com/brson/rust-anthology/blob/master/master-list.md)
* [ctjhoa/rust-learning](https://github.com/ctjhoa/rust-learning)
* [Rust Quiz](https://dtolnay.github.io/rust-quiz/)

## Error Handling and Precise Arithemtic

* [Error-Handling](./error)
* [Precise Arithmetic](./bignums)

## Concurrency, Asynchronous Computation, Multithreading

The beginning of the [substrate recipes](https://substrate.dev/recipes/base/rust.html) provides some references to the common Rust libraries for concurrency/parallelization, asynchronous computation, and multithreading:
* [Async Await](./async)
* [Concurrency](./concurrency)

## FFI
* [Foreign Function Interface](./ffi)

## Macros
* [Macros](./metaprogramming)

## WASM
* [WASM (and JS interaction with Rust)](./wasm)
* [Rust-WASM Notes](./wasm/Rusty_WASM.md)

## Rust Governance
> *[bonus](./governance)*
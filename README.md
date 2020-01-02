# learning rust

**Learning Resources**
* [The Rust Book](https://doc.rust-lang.org/book/index.html)
* [Rust by Example](https://doc.rust-lang.org/rust-by-example/index.html)

Best practices are haphazardly collected in [best](./best)

**MetaLinks**
* [cheats.rs](https://cheats.rs)
* [brson/rust-anthology](https://github.com/brson/rust-anthology/blob/master/master-list.md)
* [ctjhoa/rust-learning](https://github.com/ctjhoa/rust-learning)
* [Rust Quiz](https://dtolnay.github.io/rust-quiz/)

The beginning of the [substrate recipes](https://substrate.dev/recipes/base/rust.html) provides some references to the common Rust libraries for concurrency/parallelization, asynchronous computation, and multithreading.

# my notes (this repo)
* [Basic Recipes](./practice)
    * *Basics*
        * [`linkedlist.rs`](./practice/simple/src/linkedlist.rs): very simple linked list implementation to demonstrate the `struct`-`impl` code pattern common in Rust (from [abeinges too many lists book](http://cglab.ca/~abeinges/blah/too-many-lists/book/))
        * [`recurrence.rs`](./practice/simple/src/recurrence.rs): macros in Rust with recursion via the fibonacci sequence
        * [`conversion.rs`](./practice/simple/src/conversion.rs): practicing `From` and `Into` for conversion between types
        * [`statemachine.rs`](./practice/simple/src/statemachine.rs): Rust state machine pattern by [Hoverbear](https://hoverbear.org/2016/10/12/rust-state-machine-pattern/)
        * [`combinators.rs`](./practice/simple/src/combinator.rs): basic syntax and patterns for combinators (`map`, `and_then`)
        * [`container.rs`](./practice/simple/src/container.rs): `Box<T>` vs `Rc<T>` in the context of Trait dynamic dispatch
    * **[Code Patterns](./practice/README.md#pattern)**
    * [Serialization](./practice/serialization)
        * [basic JSON implementation with serde](./practice/serialization/serializer/src/lib.rs)
* [Error-Handling](./error)
* [Async Await](./async)
* [Concurrency](./concurrency)
* [Foreign Function Interface](./ffi)
* [Macros](./metaprogramming)
* [WASM (and JS interaction with Rust)](./wasm)
    * [notes](./wasm/Rusty_WASM.md)

> Extra: [Rust Governance](./governance)

## todo
move design to category theory rust learnings library
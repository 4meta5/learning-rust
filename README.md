# Rust Playground
An organized collection of Rust resources

> [brson/rust-anthology](https://github.com/brson/rust-anthology/blob/master/master-list.md), [ctjhoa/rust-learning](https://github.com/ctjhoa/rust-learning)

* [`Futures` and Asynchronous Patterns](./async)
    * [Resources and Notes](./async/notes.md)
* [Cryptography](./crypto)
    * [Data Structures](./crypto/primitives)
    * [Algebra](./crypto/algebra)
    * [Shamir's Secret Sharing Scheme](./crypto/erasure/ssss)
        * for conceptual background on information theory, see [notes metalink](https://github.com/AmarRSingh/notes/tree/master/Cryptography/InformationTheory)
* [Error-Handling](./error)
* [Foreign Function Interface](./ffi)
* [Macros](./metaprogramming)
* [Practice](./practice)
    * *Basics*
        * [`linkedlist.rs`](./simple/src/linkedlist.rs): very simple linked list implementation to demonstrate the `struct`-`impl` code pattern common in Rust (from [abeinges too many lists book](http://cglab.ca/~abeinges/blah/too-many-lists/book/))
        * [`recurrence.rs`](./simple/src/recurrence.rs): macros in Rust with recursion via the fibonacci sequence
        * [`conversion.rs`](./simple/src/conversion.rs): practicing `From` and `Into` for conversion between types
        * [`statemachine.rs`](./simple/src/statemachine.rs): Rust state machine pattern by [Hoverbear](https://hoverbear.org/2016/10/12/rust-state-machine-pattern/)
        * [`combinators.rs`](./simple/src/combinator.rs): basic syntax and patterns for combinators (`map`, `and_then`)
    * **[Code Patterns](./practice/README.md#pattern)**
    * [Serialization](./practice/serialization)
        * [basic JSON implementation with serde](./practice/serialization/serializer/src/lib.rs)
* [WASM (and JS interaction with Rust)](./wasm)
    * [notes](./wasm/Rusty_WASM.md)

> Extra: [Rust Governance](./governance)
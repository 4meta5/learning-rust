# Rust Playground
An organized collection of Rust resources

> [cheats.rs](https://cheats.rs), [brson/rust-anthology](https://github.com/brson/rust-anthology/blob/master/master-list.md), [ctjhoa/rust-learning](https://github.com/ctjhoa/rust-learning)

* [Error-Handling](./error)
* [Foreign Function Interface](./ffi)
* [Macros](./metaprogramming)
* [Practice](./practice)
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
* [WASM (and JS interaction with Rust)](./wasm)
    * [notes](./wasm/Rusty_WASM.md)

> Extra: [Rust Governance](./governance)
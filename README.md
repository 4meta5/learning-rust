# Rust Playground

Playground for learning Rust

* [`Futures` and Asynchronous Patterns](./async)
    * [Resources and Notes](./async/notes.md)
* [Cryptography](./crypto)
    * [Data Structures](./primitives)
    * [Algebra](./algebra)
    * [Shamir's Secret Sharing Scheme](./erasure/ssss)
        * for conceptual background on information theory, see [notes metalink](https://github.com/AmarRSingh/notes/tree/master/Cryptography/InformationTheory)
* [Error-Handling](./error)
* [Foreign Function Interface](./ffi)
* [Heap Allocation](./heap)
* [Macros](./metaprogramming)

> Extra: [Rust Governance](./governance)

**Immediate**
* rhododendron; more futures notes
* make a version of SSS that leverages futures (consensus alg)
* consider a crate for erasure encoding that leverages futures (consensus alg)

* lazy_static
* serialization with serde (play with this more)
* iterating and working with bits (maybe with respect to serialization)
* merkle tree and hashmap stuff
* ecc stuff


### Other Rust Stuff

* Category Theory for Programmers by Bartosz Milewski
* zencryption (command line encryption tool)
    1. symmetric encryption
    2. asymmetric encryption
    3. eventually, I want to implement **proxy re-encryption** in Rust (think NuCypher and Umbral)...here's parity's [implementation](https://github.com/paritytech/xpremtinel)

## Code Patterns

* [Vorner's Rust Hacks](https://vorner.github.io/2019/02/03/hacks.html)

* lazy evaluation with dynamic cacher...using [lazy-static.rs](https://github.com/rust-lang-nursery/lazy-static.rs)

* using serde with untrusted (look at Parity code and the ring library)...substrate/core/primitives/ed25519

* check out [paritytech/trie](https://github.com/paritytech/trie); try and pattern match to implement other data structures

* /metaprogramming contains notes on Rust macros and some practice code

* ffi; calling in other language code!

* wasm

* non-lexical lifetimes

#### Crates
* serde; serde_json; serde_derive
* log
* lazy_static
* ring; untrusted
* error_chain
* hex

* tokio
* futures
* fdlimit
* exit-future

* [zeroize](https://github.com/iqlusioninc/crates/tree/master/zeroize)
    * securely zero memory while avoiding compiler optimizations

###### Formal Verification

* [Rust Formal Verification Paradigm](https://www.research-collection.ethz.ch/bitstream/handle/20.500.11850/311092/paper.pdf?sequence=1&isAllowed=y)
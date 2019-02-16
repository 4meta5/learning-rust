# Cryptographic Primitives
> data structures, hashing, etc.

* [A Collection of Well-tested, serializable CDRTs (Conflict Free Data Structures)](https://github.com/rust-crdt/rust-crdt)
* [probabilistic data structures](https://github.com/crepererum/pdatastructs.rs)

**Signatures**
* [`signify-rs`](https://github.com/badboy/signify-rs) - create digital signatures for files and verify them
* [Schnorr signatures on Ristretto](https://github.com/w3f/schnorrkel)
    * [Javascript wrapper for schnorrkel signatures on Ristretto using WebAssembly](https://github.com/paritytech/schnorrkel-js)
* [Fujisaki-Suzuki Ring Signatures](https://github.com/rozbb/fujisaki-ringsig)

**Tree**
* immutable tree => use `Vec` and refer to items by index
* mutable tree => `SlotMap` crate (check it out)
* copy on write => either Rc pointers and/or `im` crate
* check out `petgraph`
* BTreeMap
* [segment tree](https://github.com/Darksonn/segment-tree)

**Graph**
* [Creating an empty iterator of a certain type in Rust](https://www.freedomlayer.org/offst/option-iterator/)

**Hash Maps**
* [The Swiss Army Knife of HashMaps](https://blog.waffles.space/2018/12/07/deep-dive-into-hashbrown/)
    * implementation of HashMap and HashSet for `no_std` environments: [Amanieu repo](https://github.com/Amanieu/hashmap_core)
* [Concurrent Hash Map](https://docs.rs/chashmap/2.2.0/chashmap/)

**Merkle Trees**
* merkle_tree [naive implementation](https://github.com/niklasad1/merkle-tree-rs)

* to implement in Rust -- [Merkle Set](https://github.com/bramcohen/MerkleSet)

**Matrix**
* [Half Matrix](https://github.com/jojolepro/half-matrix?files=1)

## Misc Resources and Repos
* [urkel tree](https://github.com/handshake-org/urkel)
* [red black merkle tree](https://github.com/amiller/redblackmerkle)
* Advanced hash tables -- [Advanced techniques to implement fast hash tables](https://attractivechaos.wordpress.com/2018/10/01/advanced-techniques-to-implement-fast-hash-tables/)
* [Hashlife](https://en.wikipedia.org/wiki/Hashlife) for Conway's Game of Life
* [Zero-Runtime-Cost Mixed List in Rust](http://nercury.github.io/rust/interesting/2015/12/12/typed-arrays.html)
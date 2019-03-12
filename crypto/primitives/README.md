# Cryptographic Primitives
> data structures, hashing, etc.

* [A Collection of Well-tested, serializable CDRTs (Conflict Free Data Structures)](https://github.com/rust-crdt/rust-crdt)
* [`BurntSushi/fst`](https://github.com/BurntSushi/fst) -- represent large sets and maps compactly with finite state transducers
* [`ratel-rust/toolshed`](https://github.com/ratel-rust/toolshed) -- `Arena` allocator along with common data structures to use with it (for when you have to create recursively nested `enum`s and don't want to heap allocate (`Box<Box<...>>`))
* [probabilistic data structures](https://github.com/crepererum/pdatastructs.rs)

* [Strobe Protocol Encryption](https://github.com/rozbb/strobe-rs)
* [`dalek-cryptography/x25519-dalek`](https://github.com/dalek-cryptography/x25519-dalek) -- X25519 elliptic curve Diffie-Hellman key exchange
* [`tendermint/kms`](https://github.com/tendermint/kms) -- key management service for Tenderment Validator nodes

* [Lioness Block Cipher](https://github.com/burdges/lioness-rs) -- [relevant paper](https://www.cl.cam.ac.uk/~rja14/Papers/bear-lion.pdf)

**Signatures**
* [`signify-rs`](https://github.com/badboy/signify-rs) - create digital signatures for files and verify them
* [Schnorr signatures on Ristretto](https://github.com/w3f/schnorrkel)
    * [`isislovecruft/davros`](https://github.com/isislovecruft/davros) -- deterministic and verifiable randomness on schnorr signatures
    * [Javascript wrapper for schnorrkel signatures on Ristretto using WebAssembly](https://github.com/paritytech/schnorrkel-js)
* [Fujisaki-Suzuki Ring Signatures](https://github.com/rozbb/fujisaki-ringsig)
* [`kZen-networks/multi-party-ecdsa`](https://github.com/KZen-networks/multi-party-ecdsa) -- {t,n}-threshold ECDSA 

**Tree**
* immutable tree => use `Vec` and refer to items by index
* mutable tree => `SlotMap` crate (check it out)
* copy on write => either Rc pointers and/or `im` crate
* check out `petgraph`
* [On-Disk B+ Tree](https://github.com/wspeirs/btree)
* [segment tree](https://github.com/Darksonn/segment-tree)
* [Bounded octree for spatial partitioning](https://github.com/Nercury/octree-rs)
* [`paritytech/trie`](https://github.com/paritytech/trie) -- Base-16 Modified Particia Merkle Tree (aka Trie)

**Merkle Trees**
* merkle_tree [naive implementation](https://github.com/niklasad1/merkle-tree-rs)
* to implement in Rust -- [Merkle Set](https://github.com/bramcohen/MerkleSet)
* [Flexible Binary Merkle Tree](https://github.com/ChosunOne/merkle_bit)
    * [blog post](https://medium.com/@niallmoore22/binary-merkle-trie-aad76f422983)
* to impl in Rust -- [Aergo State Trie](https://github.com/aergoio/aergo/tree/master/pkg/trie)
    * [blog post](https://medium.com/aergo/releasing-statetrie-a-hash-tree-built-for-high-performance-interoperability-6ce0406b12ae)


**Graph**
* [Creating an empty iterator of a certain type in Rust](https://www.freedomlayer.org/offst/option-iterator/)

**(Hash) (Maps)**
* [`RustCrypto/hashes`](https://github.com/RustCrypto/hashes) -- Collection of cryptographic hash functions
* [The Swiss Army Knife of HashMaps](https://blog.waffles.space/2018/12/07/deep-dive-into-hashbrown/)
    * implementation of HashMap and HashSet for `no_std` environments: [Amanieu repo](https://github.com/Amanieu/hashmap_core)
    * [hashbrown](https://github.com/Amanieu/hashbrown)
* [Concurrent Hash Map](https://docs.rs/chashmap/2.2.0/chashmap/)

**Matrix**
* [Half Matrix](https://github.com/jojolepro/half-matrix?files=1)

## Misc Resources and Repos
* [urkel tree](https://github.com/handshake-org/urkel)
* [red black merkle tree](https://github.com/amiller/redblackmerkle)
* Advanced hash tables -- [Advanced techniques to implement fast hash tables](https://attractivechaos.wordpress.com/2018/10/01/advanced-techniques-to-implement-fast-hash-tables/)
* [Hashlife](https://en.wikipedia.org/wiki/Hashlife) for Conway's Game of Life
* [Zero-Runtime-Cost Mixed List in Rust](http://nercury.github.io/rust/interesting/2015/12/12/typed-arrays.html)
* [`debris/ethbloom`](https://github.com/debris/ethbloom)
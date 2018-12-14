# Rust Playground

This repo is my playground for learning Rust.

**Immediate**
* rhododendron; more futures notes
* make a version of SSS that leverages futures (consensus alg)
* consider a crate for erasure encoding that leverages futures (consensus alg)

* lazy_static
* serialization with serde (play with this more)
* iterating and working with bits (maybe with respect to serialization)
* merkle tree and hashmap stuff
* ecc stuff

## Projects

### Substrate/Polkadot

* decentralized orderbook (=> prediction markets; decentralized exchanges)
    * look at 0x code...they've figured this out to an extent
    * Uniswap as well

* algorithmic dispute resolution (<=> construction contracts; supply chain tracking...reach out to Hyperledger team if I start building this)

* decentralized lending pool

* TCRs of course

> some old shit

### Other Rust Stuff

* Category Theory for Programmers by Bartosz Milewski
* zencryption (command line encryption tool)
    1. symmetric encryption
    2. asymmetric encryption
    3. eventually, I want to implement **proxy re-encryption** in Rust (think NuCypher and Umbral)...here's parity's [implementation](https://github.com/paritytech/xpremtinel)

## Code Patterns
* lazy evaluation with dynamic cacher...using [lazy-static.rs](https://github.com/rust-lang-nursery/lazy-static.rs)

* using serde with untrusted (look at Parity code and the ring library)...substrate/core/primitives/ed25519

* check out [paritytech/trie](https://github.com/paritytech/trie); try and pattern match to implement other data structures

* /metaprogramming contains notes on Rust macros and some practice code

* ffi; calling in other language code!

* wasm

### Data Structures
* [urkel tree](https://github.com/handshake-org/urkel)
* [red black merkle tree](https://github.com/amiller/redblackmerkle)
* Advanced hash tables -- [Advanced techniques to implement fast hash tables](https://attractivechaos.wordpress.com/2018/10/01/advanced-techniques-to-implement-fast-hash-tables/)
* Doubly Linked List -- [sol-dllr](https://github.com/skmgoldin/sol-dll/blob/master/contracts/DLL.sol)
* [Hashlife](https://en.wikipedia.org/wiki/Hashlife) for Conway's Game of Life
* [Zero-Runtime-Cost Mixed List in Rust](http://nercury.github.io/rust/interesting/2015/12/12/typed-arrays.html)

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
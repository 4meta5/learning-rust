# Rust Playground

This repo is my playground for learning Rust.

## Code Patterns
* lazy evaluation with dynamic cacher...using [lazy-static.rs](https://github.com/rust-lang-nursery/lazy-static.rs)

* using serde with untrusted (look at Parity code and the ring library)...substrate/core/primitives/ed25519

* errorchain (learn how to build macros using this example...it's very well made)....[link](https://github.com/rust-lang-nursery/error-chain)

### Data Structures
* [urkel tree](https://github.com/handshake-org/urkel)
* [red black merkle tree](https://github.com/amiller/redblackmerkle)
* Advanced hash tables -- [Advanced techniques to implement fast hash tables](https://attractivechaos.wordpress.com/2018/10/01/advanced-techniques-to-implement-fast-hash-tables/)
* Doubly Linked List -- [sol-dllr](https://github.com/skmgoldin/sol-dll/blob/master/contracts/DLL.sol)
* [Hashlife](https://en.wikipedia.org/wiki/Hashlife) for Conway's Game of Life

## Projects
> some old shit

* Category Theory for Programmers by Bartosz Milewski
* zencryption (command line encryption tool)
    1. symmetric encryption
    2. asymmetric encryption
    3. eventually, I want to implement **proxy re-encryption** in Rust (think NuCypher and Umbral)...here's parity's [implementation](https://github.com/paritytech/xpremtinel)
* pwasm-nft-example
    1. ecr 165
    2. surrounding solidity contracts
    * consider using/contributing to 'solc' (Solidity to Rust compiler)
* RadicalTCR

## Crates
* wasm crates
* serde
* parity
* althea
* lighthouse (rust eth 2.0 client)
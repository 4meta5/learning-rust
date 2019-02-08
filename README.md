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
> building modules is the best approach; maximize synchronous communication within chains in lieu of relatively *expensive* cross-chain communication (which is asynchronous)

> would be interesting to analyze the consensus process and consider how this could be replicated in the context of zero knowledge technology (look at Zokrates for this; or DIZK)

* decentralized orderbook (=> prediction markets; decentralized exchanges)
    * look at 0x code...they've figured this out to an extent
    * Uniswap as well
    * a dex module would be very useful

* algorithmic dispute resolution (<=> construction contracts; supply chain tracking...reach out to Hyperledger team if I start building this)

* central bank chain (some protocol not dissimilar to MakerDAO operating on a parachain) -- designed with the purpose of digitizing fiat currencies in mind
    * decentralized lending pool

* file storage chain

* IoT chain
    * nothing but the capability to send and receive messages
    * received messages are kept in a state trie for some time
    * ulta-light (low bandwith) IoT or mobile devices get proofs of finality periodically

* TCRs of course

* consider reaching out to *Prestwich* with respect to *Riemann* for collaboration on *good* architecture for cross-chain calls

* distributed collation a la Blitz protocol
    * like Mimblewimble? -- processing transactions in parallel

> some old shit

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
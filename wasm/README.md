# WASM Notes and Code
> [notes](./notes.md)

Picking up heat: 
1. **[Seed-rs](https://github.com/David-OConnor/seed)** (inspired by Elm)
    * [good examples here](https://github.com/David-OConnor)
2. **[wasmer](https://wasmer.io/) -- build once, run anywhere; universal binaries powered by WebAssembly**
* alternative to [paritytech/wasmi](https://github.com/paritytech/wasmi)
3. **[`wasm-bindgen` book](https://rustwasm.github.io/wasm-bindgen/introduction.html)**; [pretty cool interactive tutorial](https://webassembly.studio/?f=gzubao6tg3)
4. [`paritytech/wasmi`: wasm interepreter in Rust](https://github.com/paritytech/wasmi)
5. [`wasmparser.rs`: event-driven library for parsing WASM binary files](https://github.com/yurydelendik/wasmparser.rs)

*PreReqs*
* [Rust, the book](https://doc.rust-lang.org/book/)
* [HTML, CSS](https://developer.mozilla.org/en-US/docs/Learn)

> great metalink: **[Reflecting on Rust and WASM in 2018](https://rustwasm.github.io/2018/12/06/reflecting-on-rust-and-wasm-in-2018.html)**

## Getting Started
* [A Cartoon Intro to WebAssembly](https://hacks.mozilla.org/2017/02/a-cartoon-intro-to-webassembly/)
* [Understanding WebAssembly text format](https://developer.mozilla.org/en-US/docs/WebAssembly/Understanding_the_text_format)
* [Introduction to WebAssembly](https://rsms.me/wasm-intro)

* [WebAssembly Troubles part 1: WebAssembly Is Not a Stack Machine](http://troubles.md/posts/wasm-is-not-a-stack-machine/)

## Resources

 * [schnorrkel-js: a JS wrapper for schnorrkel sigs on Ristretto using WASM](https://github.com/paritytech/schnorrkel-js)

* [Threads and Atomics in WebAssembly](https://github.com/WebAssembly/threads)

* [Continuous Integration for WebAssembly](https://rustwasm.github.io/book/reference/add-wasm-support-to-crate.html#maintaining-ongoing-support-for-webassembly)
* [Testing on `wasn32-unknown-unknown` with `wasm-bindgen-test`](https://rustwasm.github.io/wasm-bindgen/wasm-bindgen-test/index.html)

* [Rust WASM frontend framework comparison](https://github.com/flosse/rust-web-framework-comparison/blob/master/README.md#frontend-frameworks-wasm)

* [WASM on the Blockchain: The Lesser Evil](https://medium.com/polkadot-network/wasm-on-the-blockchain-the-lesser-evil-da8d7c6ef6bd)

* [Rust and WebAssembly in 2019](http://fitzgeraldnick.com/2018/12/14/rust-and-webassembly-in-2019.html)

* [Interesting WebAssembly (github) issue with some compiler stuff](https://github.com/WebAssembly/design/issues/796)

* [nebulet/nebulet](https://github.com/nebulet/nebulet?files=1)
    * Nebulet is a microkernel that executes WebAssembly modules in ring 0 and a single address space to increase performance. This allows for low context-switch overhead, syscalls just being function calls, and exotic optimizations that simply would not be possible on conventional operating systems. The WebAssembly is verified, and due to a trick used to optimize out bounds-checking, unable to even represent the act of writing or reading outside its assigned linear memory.

* [React/Redux style programming in Rust](https://github.com/richardanaya/virtual-dom-rs-counter/blob/master/README.md)
* [redux-rs](https://github.com/redux-rs/redux-rs)
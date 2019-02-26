# Networking
> code patterns to be infleunced by [async/await standards progress](../notes.md)

* [libp2p notes](./libp2p.md)

## Code Patterns and Tutorials
> [*Easy Pattern*: HTTP Server is function that takes requests as input and returns a future that returns responses upon completion](https://rust-lang.github.io/async-book/getting_started/http_server_example.htmls)

* [Ferrous Systems -- TCP Server Course in Rust](https://github.com/ferrous-systems/rust-three-days-course)
* [Network simulation in Rust](https://github.com/canndrew/netsim)

* [`sorpaas/devp2p-rs`](https://github.com/sorpaas/devp2p-rs) -- Rust implementation for devp2p's Distributed Peer Table and RLPs transport protocol
* [`debris/devp2p-tokio`](https://github.com/debris/devp2p-tokio)
* [`withoutboats/romio`](https://github.com/withoutboats/romio) -- asynchronous networking primitives
* [`driftluo/p2p`](https://github.com/driftluo/p2p) -- minimal implementation of a multiplexed p2p network
* [`paritytech/yamux`](https://github.com/paritytech/yamux) - multiplexer over reliable, ordered connections


* [`tower-rs/tokio-tower`](https://github.com/tower-rs/tokio-tower) - A WIP implementation of convenience integrations between tokio and tower
* [`jonhoo/tokio-io-pool`](https://github.com/jonhoo/tokio-io-pool) - IO-oriented tokio runtime thread pool

* [`firecracker-microvm`](https://github.com/firecracker-microvm/firecracker) -- secure and fast microVMs for serverless computing

* [SIMD Intrinsics (Video)](https://www.youtube.com/watch?v=4Gs_CA_vm3o&app=desktop)
* [Crossbeam Channel](https://github.com/crossbeam-rs/crossbeam/blob/master/crossbeam-channel/README.md) -- provides multi-producer multi-consumer channels for message passing. It is an alternative to `std::sync::mpsc` with more features and better performance.

## TCP/IP

* [](https://tools.ietf.org/html/rfc1180)

## Remote Procedure Call (RPC) Stuff

* [Parity JSONRPC](https://github.com/paritytech/jsonrpc)
    * `JSON-RPC`: A standard to call functions on a remote system using a JSON protocol. For Substrate, this is implemented through the Parity JSONRPC crate.
    * `JSON-RPC Core Crate`: Allows creation of JSON-RPC server handler, with supported methods registered. Exposes the Substrate Core via different types of Transport Protocols (i.e. WS, HTTP, TCP, IPC)
    * `JSON-RPC Macros Crate`: Allows simplifying in code the creation process of the JSON-RPC server through creating a Rust Trait that is annotated with RPC method names, so you can just implement the methods names
    * `JSON-RPC Proxy Crate`: Expose a simple server such as TCP from binaries, with another binary in front to serve as a proxy that will expose all other Transport Protocols, and process incoming RPC calls before reaching an upstream server, making it possible to implement Caching Middleware (saves having to go all the way to the node), Permissioning Middleware, load balancing between node instances, or moving account management to the proxy which processes the signed transaction. This provides an alternative to embedding the whole JSON-RPC in each project and all the configuration options for each server.
    * `JSON-RPC PubSub Crate`: Custom (though conventional) extension that is useful for Dapp developers. It allows "subscriptions" so that the server sends notifications to the client automatically (instead of having to call and poll a remote procedure manually all the time) but only with Transport Protocols that support persistent connection between client and server (i.e. WS, TCP, IPC)
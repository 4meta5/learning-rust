# Networking
> code patterns to be infleunced by [async/await standards progress](../notes.md)

* [libp2p notes](./libp2p.md)
* [databases](./db.md)
* [remote procedure call stuff](./rpc.md)

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

* [RFC1180: TCP/IP Explanation](https://tools.ietf.org/html/rfc1180)
# Futures => Async/Await

* [`futures`, `async/await`](./notes.md)
* [networking](./networking)

To define a custom `future`, you typically import the following:

```rust
use futures::prelude::*;
use std::pin::Pin;
use task::{Waker, Poll};
```

## WG Coordination

* [Async Ecosystem WG](https://blog.yoshuawuyts.com/async-ecosystem-wg/) -- 2/27/2019 by YoshuaWuyts
* [Async/Await Status Report](http://smallcultfollowing.com/babysteps/blog/2019/03/01/async-await-status-report/) -- 3/1/2019 by 

## Code and References

* [The What and How of Futures and async/await in Rust](https://www.youtube.com/watch?v=9_3krAQtD2k) by JonHoo
* [Ferrous Systems -- TCP Server Course in Rust](https://github.com/ferrous-systems/rust-three-days-course)1
* *[`jonhoo/faktory-rs`](https://github.com/jonhoo/faktory-rs)* -- Rust bindings for Faktory clients and workers (may be useful for coding `async helpers`)

```rust
trait SimpleFuture {
    type Output;
    fn poll(&mut self, wake: fn()) -> Poll<Self::Output>;
}

enum Poll<T> {
    Ready(T).
    Pending,
}
```

**Future Reference Repos**
* [Farenheit](https://rust-lang-nursery.github.io/futures-rs/blog/2018/08/17/toykio.html)
* [rphmeier/honeybadger](https://github.com/rphmeier/honeybadger) -- HoneybadgerBFT in Rust
* [paritytech/rhododendron](https://github.com/paritytech/rhododendron) -- asynchronously safe BFT consensus, implementation in Rust
* [withoutboats/romio](https://github.com/withoutboats/romio) -- asynchronous networking primitives
* [jonhoo/tokio-io-pool](https://github.com/jonhoo/tokio-io-pool) -- an I/O oriented tokio runtime thread pool
* [fitzgen/state_machine_future](https://github.com/fitzgen/state_machine_future) -- easily create type-safe `Future`s from state machines

**Lock-Free**
<!--Find out what this means in the context of futures lmao, you're so far behind!!!-->
* [Bus-queueL Lock-free Bounded Non-Blocking Pub-Sub Queue](https://github.com/filipdulic/bus-queue)
    * `parking_lot`, `hash-brown`
    * [blog post](http://www.rossbencina.com/code/lockfree)
    * [jonhoo/bus](https://github.com/jonhoo/bus)
* [Rust concurrency checker](https://github.com/carllerche/loom)

## Blind Spots
* `Pin`
* `Stream`
* `Sink`
* channel mechanisms
* `futures-io`
* threadpool executor
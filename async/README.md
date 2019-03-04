# Futures => Async/Await

* [`futures`, `async/await`](./notes.md)
* [networking](./networking)

To define a custom `future`, you typically import the following:

```rust
use futures::prelude::*;
use std::pin::Pin;
use task::{Waker, Poll};
```

[SRC(https://boats.gitlab.io/blog/post/wakers-i/): The async/await pattern in Rust is comprised of three fundamental components:
1. **futures** are like a pause-able computation (really a proxy for an eventual response; compile down to state machines)
2. the **executor** schedules futures by polling them when they are ready to make progress
3. the futures depend on **event-sources** (for async IO, this is called the **reactor**). The **event-source wakes the executor** when an event occurs that will allow the future to make progress

Once a future is spawned onto an executor, that future gets executed to completion using a three phase cycle.:
1. **Poll**: The executor polls the future, which computes until it reaches a point at which it can no longer make progress.
2. **Wait**: The reactor or event source registers that the future is waiting on an event to occur. The future has returns `Poll::Pending` and the event source is now tracking that it will need to wake this future when that event is ready.
3. **Wake**: The event happens and the future is woken up. It is now up to the executor to schedule the future to be polled again.

The asynchronous **event loop** <br>
**Executor** manages the program's compute resources + **reactor** manages the program's IO resources

## WG Coordination

* [Async Ecosystem WG](https://blog.yoshuawuyts.com/async-ecosystem-wg/) -- 2/27/2019 by YoshuaWuyts
* [Async/Await Status Report](http://smallcultfollowing.com/babysteps/blog/2019/03/01/async-await-status-report/) -- 3/1/2019 by 

## Code and References

* [The What and How of Futures and async/await in Rust](https://www.youtube.com/watch?v=9_3krAQtD2k) by JonHoo
* [Ferrous Systems -- TCP Server Course in Rust](https://github.com/ferrous-systems/rust-three-days-course)1
* *[`jonhoo/faktory-rs`](https://github.com/jonhoo/faktory-rs)* -- Rust bindings for Faktory clients and workers (may be useful for coding `async helpers`)

**Future Reference Repos**
* [Farenheit](https://rust-lang-nursery.github.io/futures-rs/blog/2018/08/17/toykio.html)
* [rphmeier/honeybadger](https://github.com/rphmeier/honeybadger) -- HoneybadgerBFT in Rust
* [paritytech/rhododendron](https://github.com/paritytech/rhododendron) -- asynchronously safe BFT consensus, implementation in Rust
* [withoutboats/romio](https://github.com/withoutboats/romio) -- asynchronous networking primitives
* [jonhoo/tokio-io-pool](https://github.com/jonhoo/tokio-io-pool) -- an I/O oriented tokio runtime thread pool
* [fitzgen/state_machine_future](https://github.com/fitzgen/state_machine_future) -- easily create type-safe `Future`s from state machines

## Blind Spots
* `Pin`
* `Stream`
* `Sink`
* channel mechanisms
* `futures-io`
* threadpool executor
# Futures => Async/Await
> [`futures`, `async/await`](./notes.md), [networking](./networking.md)

* [The What and How of Futures and async/await in Rust](https://www.youtube.com/watch?v=9_3krAQtD2k) by JonHoo

```
trait SimpleFuture {
    type Output;
    fn poll(&mut self, wake: fn()) -> Poll<Self::Output>;
}

enum Poll<T> {
    Ready(T).
    Pending,
}
```

* HoneyBadgerBFT implementation (by rphmeier)
* tokyio (Farenheit) -- simple futures executor for learning purposes

**Future Reference Repos**
* [Farenheit](https://rust-lang-nursery.github.io/futures-rs/blog/2018/08/17/toykio.html)
* [rphmeier/honeybadger](https://github.com/rphmeier/honeybadger) -- HoneybadgerBFT in Rust
* [paritytech/rhododendron](https://github.com/paritytech/rhododendron) -- asynchronously safe BFT consensus, implementation in Rust
* [withoutboats/romio](https://github.com/withoutboats/romio) -- asynchronous networking primitives
* [jonhoo/tokio-io-pool](https://github.com/jonhoo/tokio-io-pool) -- an I/O oriented tokio runtime thread pool

**Lock-Free**
<!--Find out what this means in the context of futures lmao, you're so far behind!!!-->
* [Lock-free Bounded Non-Blocking Pub-Sub Queue](https://github.com/filipdulic/bus-queue)
    * `parking_lot`, `hash-brown`
    * [blog post](http://www.rossbencina.com/code/lockfree)
    * [jonhoo/bus](https://github.com/jonhoo/bus)

**Pin Reading**
* [withoutboats/async_self_referential_structs](https://boats.gitlab.io/blog/post/2018-01-25-async-i-self-referential-structs/)
* [A Formal Look at Pinning](https://www.ralfj.de/blog/2018/04/05/a-formal-look-at-pinning.html)

## Blind Spots
* `Pin`
* `Stream`
* `Sink`
* channel mechanisms
* `futures-io`
* threadpool executor
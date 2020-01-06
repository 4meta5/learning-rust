# Learning Resources

* [tokio docs](https://tokio.rs/)
* [romio](https://github.com/withoutboats/romio)

## TutorialsQ

* [Snoyman Tutorial](https://www.snoyman.com/blog/2018/12/rust-crash-course-07-async-futures-tokio)

* [Async Book by Aturon](https://rust-lang.github.io/async-book/execution/future.html)
* [Async borrowing by Turon](http://aturon.github.io/2018/04/24/async-borrowing/)

* [async-await-final by @withoutboats](https://boats.gitlab.io/blog/post/2018-04-06-async-await-final/)
* [Making progress in await syntax by @withoutboats](https://boats.gitlab.io/blog/post/await-syntax/)

* [Async in Rust, circa 2018](https://rust-lang-nursery.github.io/wg-net/2018/12/13/async-update.html)
* [Why Rust's async functions should use the outer return type approach](https://github.com/MajorBreakfast/rust-blog/blob/master/posts/2018-06-19-outer-return-type-approach.md)

**Tokio**
* [Tokio Async Protocol](https://leshow.github.io/post/impl_proto_tokio/?utm_source=share&utm_medium=ios_app)
* [Tokio Internals](https://cafbit.com/post/tokio_internals/) -- very comprehensive

## Code
* **[Hexilee/async-io-demo](https://github.com/Hexilee/async-io-demo)**
* [async-await streaming hyper-body example](https://github.com/tokio-rs/tokio/blob/master/tokio-async-await/examples/src/hyper.rs)
* [warp](https://seanmonstar.com/post/181223452087/warp-v0110)

* [Juliex](https://github.com/withoutboats/juliex) - a simple futures *executor*
    * implemented as a threadpool executor using a single, shared queue. Algorithmically, it is very similar to the Threadpool executor provided by the futures crate. The main difference is that juliex uses a crossbeam channel and performs a single allocation per spawned future, whereas the futures Threadpool uses std concurrency primitives and multiple allocations.

* [`exit-future`](https://github.com/paritytech/exit-future) -- Future that resolves when exit signal is set
* [`aio-limited`](https://github.com/paritytech/aio-limited) -- rate limiting for `AsyncRead` and `AsyncWrite` types
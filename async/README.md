# Futures => Async/Await
> [notes](./futures.md)

* [The What and How of Futures and async/await in Rust](https://www.youtube.com/watch?v=9_3krAQtD2k) by JonHoo

```
pub trait Future {
    type Item;
    type Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error>;
}

pub enum Async<T> {
    Ready(T),
    Pending,
}
```


**Reference Repos**
* [rphmeier/honeybadger](https://github.com/rphmeier/honeybadger) -- HoneybadgerBFT in Rust
* [paritytech/rhododendron](https://github.com/paritytech/rhododendron) -- asynchronously safe BFT consensus, implementation in Rust
* [withoutboats/romio](https://github.com/withoutboats/romio) -- asynchronous networking primitives
* [jonhoo/tokio-io-pool](https://github.com/jonhoo/tokio-io-pool) -- an I/O oriented tokio runtime thread pool

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
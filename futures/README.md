# Futures
> Futures provide a robust way of handling asynchronous computation

> [Informal intro to Futures and Tokio by Jon Hoo (Gjengset)](https://www.youtube.com/watch?v=9_3krAQtD2k)

## What are Futures
> [docs](https://docs.rs/futures/0.1.25/futures/)

*Futures are a concept for an object which is a proxy for another value that may not be ready yet. With an object representing a value that will eventually be available, futures allow for powerful composition of tasks through basic combinators that can perform operations like chaining computations, changing the types of futures, or waiting for two futures to complete at the same time.*

**What is an executor?**
An executor ensures that the future is executed. In practice, this works by `poll`ing all the futures until they return `Async::Ready`. Here is the definition of `Future::poll` method:

```
type Poll<T, E> = Result<Async<T>, E>;
```
The return type of the `Future::poll` method, indicates whether a future's value is ready or not. This returns
* `Ok(Async::Ready(t))` if a future has successfully resolved
* `Ok(Async::NotReady)` if a future is not ready to complete yet
* `Err(e)` if a future has completed with the given failure

## Tokio
> An implementation of an executor and the other things you need to write asychronous applications in Rust.

Tokio enables notifications to *wake up* after `Async::NotReady` is returned a `poll()` call and we are waiting for it to turn into `Async::Ready`. In Tokio, this is called a `Reactor` (it is essentially a handle to something that tells you when the OS is ready).

## Async/Await
> going to be implemented in the Rust programming language soon

* [Turon Blog Post](http://aturon.github.io/2018/04/24/async-borrowing/)
* [@withoutboat blog post](https://boats.gitlab.io/blog/post/2018-04-06-async-await-final/)
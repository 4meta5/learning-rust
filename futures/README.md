# Futures
> Futures provide a robust way of handling asynchronous computation

> [Informal intro to Futures and Tokio by Jon Hoo (Gjengset)](https://www.youtube.com/watch?v=9_3krAQtD2k)

* [Intro](#intro)
    * [Use Cases](#examples)
    * [Streams](#streams)
* [Tokio](#tokio)
* [Async/Await](#async)

## What are Futures <a name = "intro"></a>
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

### Examples and Features <a name = "examples"></a>
> [relevant Turon Blog Post](http://aturon.github.io/2016/08/11/futures/)

**Examples**
* **database query** that's executing in a thread pool. When the query finishes, the future is completed, and its value is the result of the query.
* **An RPC Invocation** to a server. When the server replies, the future is completed, and its value is the server's response.
* **A timeout**. When the time is up, the future is completed, and its value is just `()`
* **A long-running CPU-intensive task**, running on a thread pool. When the task finishes, the future is completed, and its value is the return value of the task.
* **Reading bytes from a socket**. When the bytes are ready, the future is completed -- and depending on the buffering strategy, the bytes might be returned directly, or written as a side-effect into some existing buffer.

Things become very interesting with futures when you combine them:
* **Sequential composition**: `f.and_then(|val| some_new_future(val))`. Gives you a future that executes the future `f`, takes the `val` it produces to build another future `some_new_future(val)` and then executes that future.
* **Mapping**: `f.map(|va| some_new_value(val))`. Gives you a future that executes the future `f` and yields the result of `some_new_value(val)`.
* **Joining**: `f.join(g)`. Gives you a future that executes the futures `f` and `g` in parallel, and completes when *both* are complete, returning both of their values.
* **Selecting**: `f.select(g)`. Gives you a future that executes the future `f` and `g` in parallel, and completes when *one* of them is complete, returning both of their values.

### Streams <a name = "streams"></a>

*Futures are all about a single value that will eventually be produced, but many event sources naturally produce a stream of values over time...The futures library includes a `Stream` trait such that the set up produces a sequence of values over time. It has a set of combinators, some of which work with futures.*

## Tokio <a name = "tokio"></a>
> [Expanding the Tokio Project](http://aturon.github.io/2016/08/26/tokio/) by Aaron Turon

Tokio is essentially an implementation of an executor and the other things you need to write asychronous applications in Rust.

Tokio enables notifications to *wake up* after `Async::NotReady` is returned a `poll()` call and we are waiting for it to turn into `Async::Ready`. In Tokio, this is called a `Reactor` (it is essentially a handle to something that tells you when the OS is ready).

**Layers of Abstraction**
* the `tokio-service` crate provides core trait definitions for services. A **service** is a function from requests to futures of responses (therefore, building an http server is just a matter of writing a function from http requests to futures of http responses). CHECK OUT [YOUR SERVER AS A FUNCTION](https://monkey.org/~marius/funsrv.pdf) for inspiration! 

## Async/Await <a name = "async"></a>
> going to be implemented in the Rust programming language soon

*What if we want to handle a large number of simultaneous connections, many of which are waiting for I/O, but we want to keep the number of OS threads to a minimum? **The answer is Asynchronous I/O***.

In Asynchronous I/O, we can *attempt* an I/O operation without blocking; if it can't complete immediately, you can retry at some later point.

* [Turon Blog Post](http://aturon.github.io/2018/04/24/async-borrowing/)
* [@withoutboat blog post](https://boats.gitlab.io/blog/post/2018-04-06-async-await-final/)
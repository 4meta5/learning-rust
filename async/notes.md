# Notes

* [Futures](#intro)
    * [Use Cases](#examples)
    * [Streams](#streams)
* [Tokio](#tokio)
* [Async/Await](#async)
    * [Async IO](#io)

**ReadingQ**
* [Tokio Internals](https://cafbit.com/post/tokio_internals/) -- very comprehensive

* [Async Book Chapter on Futures](https://rust-lang.github.io/async-book/execution/future.html)

* [Async borrowing by Turon](http://aturon.github.io/2018/04/24/async-borrowing/)
* [async-await-final by @withoutboats](https://boats.gitlab.io/blog/post/2018-04-06-async-await-final/)
* [Making progress in await syntax by @withoutboats](https://boats.gitlab.io/blog/post/await-syntax/)
* [Async in Rust, circa 2018](https://rust-lang-nursery.github.io/wg-net/2018/12/13/async-update.html)
* [Why Rust's async functions should use the outer return type approach](https://github.com/MajorBreakfast/rust-blog/blob/master/posts/2018-06-19-outer-return-type-approach.md)

* [Hexilee/async-io-demo](https://github.com/Hexilee/async-io-demo) -- great demo!
* [async-await streaming hyper-body example](https://github.com/tokio-rs/tokio/blob/master/tokio-async-await/examples/src/hyper.rs)
* [warp](https://seanmonstar.com/post/181223452087/warp-v0110) -- solid example code

## Futures <a name = "intro"></a>
> [docs](https://docs.rs/futures/0.1.25/futures/)

*Futures provide a robust way of handling asynchronous computation*

> [Informal intro to Futures and Tokio by Jon Hoo (Gjengset)](https://www.youtube.com/watch?v=9_3krAQtD2k)

*Futures are a concept for an object which is a proxy for another value that may not be ready yet. With an object representing a value that will eventually be available, futures allow for powerful composition of tasks through basic combinators that can perform operations like chaining computations, changing the types of futures, or waiting for two futures to complete at the same time.*

> *In essence, a future represents a value that might not be ready yet. Usually the future becomes complete (the value is ready) due to an event happening somewhere else.*

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
* **Mapping**: `f.map(|val| some_new_value(val))`. Gives you a future that executes the future `f` and yields the result of `some_new_value(val)`.
* **Joining**: `f.join(g)`. Gives you a future that executes the futures `f` and `g` in parallel, and completes when *both* are complete, returning both of their values.
* **Selecting**: `f.select(g)`. Gives you a future that executes the future `f` and `g` in parallel, and completes when *one* of them is complete, returning both of their values.

### Streams <a name = "streams"></a>

*Futures are all about a single value that will eventually be produced, but many event sources naturally produce a stream of values over time...The futures library includes a `Stream` trait such that the set up produces a sequence of values over time. It has a set of combinators, some of which work with futures.*

### Design Rationale <a name = "design"></a>
> [Futures Design by Aaron Turon](http://aturon.github.io/2016/09/07/futures-design/)

* leverage Rust's traits and closures for ergonomics and cost-avoidance; traits and closures do not require heap allocation or dynamic dispatch
* core `Future` abstraction is *demand-driven* rather than callback-oriented (ie follow the "readiness" style rather than the "completion" style)
* providing a task abstraction, similar to a green thread, that drives a future to completion 

> Rather than opting for the *completion-based* approach, in which events are signaled based on completion of operations, the Rust implementation of `Future`s is "demand-driven"(*readiness-based*). 

```
/// A simplified version of the trait, without error-handling
trait Future {
    // the type of value produced on success
    type Item;

    // Polls the future, resolving to a value if possible
    fn poll(&mut self) -> Async<Self::Item>;
}

enum Async<T> {
    // Represents that a value is immediately ready
    Ready(T);

    // Represents a value that is not ready yet, but may be so later
    NotReady;
}
```

Rather than the future proactively invoking a callback on completion, an external party must *poll* the future to drive it to completion.

A *task* is a future that is being executed. The task blocks by yielding back to its executor, after installing itself as a callback for the event it's waiting on. Under this paradigm, the task is woken up when the future is ready. at which point it will re-`poll` the future. Task instance stays fixed for the lifetime of the future it is executing -- so no allocation is needed to create or install this callback.

Tasks provide a `park`/`unpark` API for blocking and wakeup:
```
/// Returns a handle to the current task to call unpark at a later date
fn park() -> Task;

impl Task {
    /// Indicate that the task should attempt to poll its future in a timely fashion
    fn unpark(&self);
}
```

Blocking a future is a matter of using `park` to get a handle to its task, putting the resulting `Task` in some wakeup queue for the event of interest, and returning `NotReady`. When the event of interest occurs, the `Task` handle can be used to wake back up the task, e.g. by rescheduling it for execution on a thread pool.

> The future within a task compiles down to a state machine, so that every time the task wakes up to continue polling, it continues execution from the current state.

## Tokio <a name = "tokio"></a>
> [Expanding the Tokio Project](http://aturon.github.io/2016/08/26/tokio/) by Aaron Turon

Tokio is essentially an implementation of an executor and the other things you need to write asychronous applications in Rust.

Tokio enables notifications to *wake up* after `Async::NotReady` is returned a `poll()` call and we are waiting for it to turn into `Async::Ready`. In Tokio, this is called a `Reactor` (it is essentially a handle to something that tells you when the OS is ready).

**Layers of Abstraction**
* the `tokio-service` crate provides core trait definitions for services. A **service** is a function from requests to futures of responses (therefore, building an http server is just a matter of writing a function from http requests to futures of http responses). CHECK OUT [YOUR SERVER AS A FUNCTION](https://monkey.org/~marius/funsrv.pdf) for inspiration! 

## Async/Await <a name = "async"></a>

Async functions return immediately when they are called -- none of the code in their body is executed. They return a future, representing the state machine of their body transitioning from async to await until it finally returns the final value. 

*You always know that none of the body of the async function will be evaluated until you begin polling the future it returns.*

> "*async/await is not just about avoiding combinators; it completely changes the game for borrowing*" - [Turon](http://aturon.github.io/2018/04/24/async-borrowing/)

**Why do we need `async/await` if we have futures?**
Although the `Future` trait does not explicitly impose a `'static` bound, futures have to be `'static` because they are not tied to any particular stack frame. This means that futures-based APIs are forced to take ownership of whatever they need, thereby coercing unidiomatic patterns, including threading through ownership as well as the overuse of `Rc` and `RefCell`.

async/await enables the programmer to `await` a future with borrowed data, while still being `'static` overall (@withoutboats refers to this as "borrowing across yield points")...this proposal enables fully idiomatic Rust code that runs asynchronously.

### Async IO <a name = "io"></a>

*What if we want to handle a large number of simultaneous connections, many of which are waiting for I/O, but we want to keep the number of OS threads to a minimum? **The answer is Asynchronous I/O**.

In Asynchronous I/O, we can *attempt* an I/O operation without blocking; if it can't complete immediately, you can retry at some later point.
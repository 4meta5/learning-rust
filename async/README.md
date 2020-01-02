# Asynchronous Programming

*Futures are a concept for an object which is a proxy for another value that may not be ready yet. With an object representing a value that will eventually be available, futures allow for powerful composition of tasks through basic combinators that can perform operations like chaining computations, changing the types of futures, or waiting for two futures to complete at the same time.*

> *In essence, a future represents a value that might not be ready yet. Usually the future becomes complete (the value is ready) due to an event happening somewhere else.*

**[Introduction to Async/Await Programming (withoutboats/wakers-i):](https://boats.gitlab.io/blog/post/wakers-i/)**
A running programming using async/await correctly involves three fundamental components:

The bulk of the program consists of futures, which are a kind of pause-able computation. In general, the end user’s code is will consist of futures, which they will write as if they were normal functions using async/await syntax.
* At the “top” of the program is the executor. The executor schedules futures by polling them when they are ready to make progress.
* At the “bottom” of the program, the futures depend on event sources. (in the case of async IO, the source of IO events is often called the reactor). The event source wakes the executor when an event occurs that will allow a future depending on it to make progress.
* Once a future is spawned onto an executor, that future gets executed to completion using a three phase cycle:

1. Poll: The executor polls the future, which computes until it reaches a point at which it can no longer make progress.
2. Wait: The reactor or event source registers that this future is waiting on an event to happen. The future has returned Poll::Pending and the event source is now tracking that it will need to wake this future when that event is ready.
3. Wake: The event happens, and the future is woken up. It is now up to the executor to schedule the future to be polled again.
At a high level you can think of the executor as managing the program’s compute resources (scheduling futures that are awake and ready to be polled) and the reactior as managing the program’s IO resources (waiting on IO events and waking futures when they are ready). The executor and the reactor form the two halves of what in most asynchronous computing systems is called the “event loop.” One of the great things about Rust’s design is that you can combine different executors with different reactors, instead of being tied down to one runtime library for both pieces.

**There should be many applications of this emerging primitive ([src](http://aturon.github.io/2016/08/11/futures/)):**
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
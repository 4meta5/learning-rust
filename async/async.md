# Async/Await Notes
> going to be implemented in the Rust programming language soon

Async functions return immediately when they are called -- none of the code in their body is executed. They return a future, representing the state machine of their body transitioning from async to await until it finally returns the final value. 

*You always know that none of the body of the async function will be evaluated until you begin polling the future it returns.*

> "*async/await is not just about avoiding combinators; it completely changes the game for borrowing*" - [Turon](http://aturon.github.io/2018/04/24/async-borrowing/)

**Why do we need `async/await` if we have futures?**
Although the `Future` trait does not explicitly impose a `'static` bound, futures have to be `'static` because they are not tied to any particular stack frame. This means that futures-based APIs are forced to take ownership of whatever they need, thereby coercing unidiomatic patterns, including threading through ownership as well as the overuse of `Rc` and `RefCell`.

async/await enables the programmer to `await` a future with borrowed data, while still being `'static` overall (@withoutboats refers to this as "borrowing across yield points")...this proposal enables fully idiomatic Rust code that runs asynchronously.

## Async IO

*What if we want to handle a large number of simultaneous connections, many of which are waiting for I/O, but we want to keep the number of OS threads to a minimum? **The answer is Asynchronous I/O**.

In Asynchronous I/O, we can *attempt* an I/O operation without blocking; if it can't complete immediately, you can retry at some later point.

### References

* [Async borrowing by Turon](http://aturon.github.io/2018/04/24/async-borrowing/)
* [async-await-final by @withoutboats](https://boats.gitlab.io/blog/post/2018-04-06-async-await-final/)
* [Making progress in await syntax by @withoutboats](https://boats.gitlab.io/blog/post/await-syntax/)
* [Async in Rust, circa 2018](https://rust-lang-nursery.github.io/wg-net/2018/12/13/async-update.html)
* [Why Rust's async functions should use the outer return type approach](https://github.com/MajorBreakfast/rust-blog/blob/master/posts/2018-06-19-outer-return-type-approach.md)

* [Hexilee/async-io-demo](https://github.com/Hexilee/async-io-demo) -- great demo!
* [async-await streaming hyper-body example](https://github.com/tokio-rs/tokio/blob/master/tokio-async-await/examples/src/hyper.rs)
* [warp](https://seanmonstar.com/post/181223452087/warp-v0110) -- solid example code
# Fearless Concurrency? Understanding Concurrent Programming Safety in Real-World Rust Software
> [arxiv link](https://arxiv.org/pdf/1902.01906.pdf)

**OSS Projects**
* Servo (web browser)
* TiKV (key-value storage)
* Rand (random number generation)

**`Arc<Mutex<T>>` Pattern**
To allow multiple threads to update the same copy, a shared variable has to be declared with both `Arc` and `Mutex`. To access the shared variable, a thread needs to invoke the lock function of the shared variable's protecting `Mutex`. A reference of the shared variable is returned after successfully invoking the lock function, so that the thread can read or write the shared variable. Mutual exclusion provided by `Mutex` guarantees that there is at most one thread accessing a shared value at any time.

*Thread Synchronization*
* `Mutex`
* `RwLock`
* `Condvar` - Condition variables represent the ability to block a thread such that it consumes no CPU time while waiting for an event to occur. Condition variables are typically associated with a boolean predicate (a condition) and a mutex. The predicate is always verified inside of the mutex before determining that a thread must block. [docs](https://doc.rust-lang.org/std/sync/struct.Condvar.html)
* `atomic`
* `Barrier` - A barrier enables multiple threads to synchronize the beginning of some computation. [docs](https://doc.rust-lang.org/std/sync/struct.Barrier.html)
# Lock Contention

**IDEAS**
* parallel algorithm for generating random numbers using a Mersenne Twister implementation with locks (basically a Poisson process)
* roll your own lightweight mutex (look at Rust `std` source code and build your own mutex in contex...[inspiration](https://preshing.com/20120226/roll-your-own-lightweight-mutex/))

## RwLock

*[Slow reader blocking in reader/writer locks](https://blog.nelhage.com/post/rwlock-contention/)*

For reader/writer locks, slow readers and a small number of writers (even a single writer) can lead to substantial latency spikes.

`RwLock`s aim to provide higher concurrency than a traditional mutex by allowing multiple readers to proceed in parallel; if a single writer holds the lock, no other thread may concurrently hold it for either read or write access.

**Preventing Writer Starvation**
To avoid starving writers, most read/writer lock implementations are *writer priority* `=>` once a writer starts waiting on the lock, no future reader may acquire the lock until after the writer has acquired and dropped the lock.
* without this, readers would continually overlap their sessions (never leaving a moment at which a writer can acquire an exclusive lock)

This writer priority creates the possibility of *readers blocking on other readers*; consider the following:
1. at time `T_1`, a long-running reader `R_1` acquires a read lock
2. at `T_2`, writer `W` attempts to acquire a writer lock
3. at `T_3`, reader `R_2` attempts to acquire a read lock
4. at `T_4`, `R_1` drops the read lock

Because the writer `W` has priority, `R_2` is blocked until `W` can acquire and then release the lock. However, `W` is blocked until `R_1` releases the read lock at `T_4` thereby blocking `R_2` for `T_3` and `T_4` (s.t. `R_2` is basically waiting for `R_1` to complete, but `RwLock`s are supposed to provide concurrent access).

With this in mind, a very small amount of write load can be sufficient to allow long-running readers to halt all other threads until they complete.

*If you have to use a reader/writer lock, think carefully about how long a reader or writer can hold the lock, and instrument the system such that long-lived critical sections are observable before they become problematic* (suggestions in the [src](https://blog.nelhage.com/post/rwlock-contention/))

## Resources

* [Locks aren't slow; Lock contention is](https://preshing.com/20111118/locks-arent-slow-lock-contention-is/)
* [Always Use a Lighteweight Mutex](https://preshing.com/20111124/always-use-a-lightweight-mutex/)
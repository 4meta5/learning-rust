# Nomicon Notes on Concurrency and Parallelism
> **[Chapter 8: Concurrency and Parallelism](https://doc.rust-lang.org/nomicon/concurrency.html)**

Safe Rust guarantees an absence of data races, which are defined as:
* two or more threads concurrently accessing a location of memory
* one of them is write
* one of them is unsynchronized

Data races are mostly prevented through Rust's ownership system; it's impossible to alias a mutable reference so it's impossible to perform a data race.

A type is `Send` if it is safe to send it to another thread; a type is `Sync` if it is safe to share between threads (`&T` is `Send`).

`Send` and `Sync` are automatically derived types, which essentially means that if they are composed of `Send` or `Sync` types, then they are `Send` or `Sync`.

Major types that aren't `Send` or `Sync` include:
1. raw pointers (because they have no safety guards)
2. `UnsafeCell` isn't `Sync` => `Cell`, `RefCell` are not `Sync`
3. `Rc` (refcount is shared and unsynchronized)
# Rust at Speed -- building a fast concurrent database
> by Jon Gjengset

For most applications, most operations are reads (or `SELECT`s in the database)
* if 90% of operations are reads, that's a lot of cycles
* a quick fix is caching, but there exist problems
* every time you write, you invalidate the appopriate cache away, but you don't want to throw your whole cache away

**Thundering Herd**: skewed population of popularity in your database => it might see a lot of writes and a lot of reads => you get a write for a popular key, you invalidate cache, all of the popular reads start missing because they were all for the popular key => they all go to your database at the same time => database falls over

* Facebook paper on MemCacheD Caches

**Noria** strives to answer the question, "database has your queries so why can't it maintain your cache" by updating the cache results upon writes.

* need mechanism for **Partial Materialization Problem**

## Ownership in Rust

If you *own* something, you are responsible for that thing. If you own memory, you are responsible for freeing that memory when it is responsible to do so.

Rust extends this such that if you own something, you get to choose who has access to that resource and how.

For some type `T`, you can have: `T` (owned), `&mut T` (exclusive), and `&T` (shared).

Rust will at compile time check that you haven't violated the contracts. For any two variables, you don't have a mutable reference and immutabel reference at the same time, and you also can't have multiple mutable references.

The compiler requires you to prove that you don't have data races. **If you never have something modify a thing while it's being read or modified, you cannot have data races.** So, you'll either have multiple readers or a single writer.

You also guarantee that you can only free things because the owner is responsible for freeing things and you only have one owner. The borrowchecker also checks that you haven't used anything after you've gotten rid of it.

The borrowchecker does this by adding this notion of a lifetime. If you borrow any `T`, that borrow of `T` is assigned a lifetime. You can think of the lifetime as how long you're allowed to access `T` for -- for example, if `T` lives on the stack, the lifetime that's given out for any borrow of `T` is going to be tied to the stack frame of the thing that has `T`. The compiler will check that when `T` goes away => that stack frame is popped and `T` is freed, there are no oustanding references to `T`. So if you tried to take a reference to something stored to the stack and give it to another thread, the compiler would say no, that thread can live longer than this stack frame so your program is not safe and will not compile.

We can frame concurrency problems in terms of ownership. Rust forces us to use synchronization.

### Mutex

Rust will not allow you to access a shared value unless you use something like a `Mutex`, something that provides mutable access to something that is shared.

A mutex provides safe mutations on shared data.

```rust
impl Mutex<T> {
    fn lock(&'mtx self) -> MutexGuard<'mtx, T> {}
}

impl MutexGuard<'mtx, T> {
    fn get(&'a mut self) -> &'a mut T {}
}
```

The `Mutex<T>` wraps that `T`. So you have no way of getting at `T` without also taking the lock. When you lock the `Mutex`, it takes an immutable reference to self and you get back a `MutexGuard`, which is like credential that you're currently holding the lock. So when you take the lock, you take an immutable reference to the mutex and get back a handle to the inside of the lock. That inside of the lock has a `get` method such that as long as you have exclusive access to the credential, then you have exclusive access to `T`. As long as the mutex is still alive and you still have the lock, you can get a mutable reference to whatever is inside the lock. 

This gives us mutable access to `T` through a shared reference to the mutex. It does this by mutual exclusion. Internally, the mutex knows that as long as it does its bookkeeping right, because no one else is going to have a mutable reference to `T` because they wouldn't have the lock, wouldn't have the credential so they wouldn't be able to access `T`.

Mutexes are slow because they force sequential access. You either have to do a read or a write for any unit of time. Mutexes are neither concurrent nor fast.

### RwLock

`RwLock` -- basic idea is that you can have multiple readers take a lock at the same time or have a single writer take the mutex, but you cannot have both. Either a single mutable pointer or many immutable ones. It allows for many reads in parallel. 

```rust
struct RwLock {
    r: Mutex<usize>,
    g: Mutex<()>,
}

impl RwLock {
    fn lock(&self) {
        let mut b = self.r.lock();
        *b += 1;
        if *b == 1 { self.g.lock() }
        b.unlock();
    }

    fn unlock(&self) {
        let mut b = self.r.unlock();
        *b -= 1;
        if *b == 0 { self.g.unlock() }
        b.unlock();
    }
}
```

The basic thing to observe is whenever you take the reader part of the lock, you have to take this exclusive lock first. You don't take it for very long -- you release it before you return. You have to take it whenever you take the reader handle and whenever you give it back. This means that it is ok if you have a long, criticial section because taking that lock is part of your workload.

Long critical sections => lock time irrelevant => lots of concurrency

Short critical section => all time spent in the lock which is sequential

Short critical sections => taking the reader lock will be your bottleneck. This makes it so that your program is sequential up. All we're doing when we're taking the reader lock is doing HashMap lookups so we're basically benchmarking the performance of taking a lock,

`RwLock` does not scale.

### Locks

Locks provide a safe wrapper around aliased or shared pointers; gives mutable access through a shared pointer.

Aliasing brings data races. You might have two things that point to the same thing and try to mutate it -- which is why there is unsafety in `C`.

Writing `unsafe` code is really just writing `C` code. Whenever you type the `unsafe` keyword, Rust gives you the power to do the things you can do in `C` => **dereferencing raw pointers**.

Raw pointer to T:
```rust
*mut T
```
Pointer to `T` with no lifetime. Unsafe allows you to take one of these turn it into a mutable reference to `T`. That means that you can give it out with the contract that ensure exclusive access. Basically pointer type casting in `C`.

The idea of unsafe is that you're now responsible for ensuring there are no data races because there is no additional contract with the pointer anymore.

This allows you to narrow an audit for data races to all the places that you use `unsafe`.

## EVMAP

By simply not using locks and implementing two maps simultaneously, one for readers and one for writes, swap both maps on every write, we can be sure that there are no data races and update the maps appropriately.

By invoking some unsafe Rust code to switch the mutable pointers, EVMAP outperforms lock-based maps while not sacrificing any safety when we examine the underlying unsafe Rust logic. This optimization is entirely achieved via closer proximity to the machine code. 

**IDEA**: I think examining the use of this sort of data structure in the context of CRDTs could be cool -- using this in an implementation as an optimization. 

The *Rust learning curve* is learning borrow checking.
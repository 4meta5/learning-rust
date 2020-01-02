# Queueing Theory (Concurrency) 

*Notes on Papers*
* [A Methodology for Creating Fast Wait-Free Data Structures](./fast_slow.md)
* [Fearless Concurrency? Understanding Concurrent Programming Safety in Real-World Rust Software](./fearless.md)

*Dope MetaLinks*
* **[Fear and Loathing in Lock-Free Programming](https://medium.com/@tylerneely/fear-and-loathing-in-lock-free-programming-7158b1cdd50c)**
* [Lock-free Rust: Crossbeam in 2019](https://stjepang.github.io/2019/01/29/lock-free-rust-crossbeam-in-2019.html)

## Channels
*Share your sender with other threads/components and keep your receiver to yourself*

The point of channels is how they allow you to structure isolated concurrent components. A common way of doing so is to move clones of `Sender` to other components while keeping the corresponding `Receiver` hidden. The logic is run sequentially by running an event-loop driven by incoming messages on the `Receiver`.

> *Each clone of a sender is "owned" by a single component, and you have only a single receiver, also owned by a single component*

**Common Usage Pattern**
* make clones of a `Sender` available to each component that it wants to monitor
* use those `Sender`s to send messages indicating the start of an activity
* *tailing the logs* of concurrent components with the messages in the channel representing a "stream" of logs (*this reminds me of the `consensus` following the relay chain with futures in `cumulus`)

**[Rust concurrency patterns: Still communicating by moving senders (like it's 2018)](https://medium.com/@polyglot_factotum/rust-concurrency-patterns-communicate-by-sharing-your-sender-re-visited-9d42e6dfecfa)**

## Lock-Free Fundamentals

*[Lock-Free vs Wait-Free (StackOverflow)](https://stackoverflow.com/questions/4211180/examples-illustration-of-wait-free-and-lock-free-algorithms)*
* If a program is **lock-free**, it basically means that at least one of its threads is guaranteed to make progress over an arbitrary period of time. If a program deadlocks, none of its threads (and therefore the program as a whole) cannot make progress - we can say it's not lock-free. Since lock-free programs are guaranteed to make progress, they are guaranteed to complete (assuming finite execution without exceptions).
* **Wait-free** is a stronger condition which means that every thread is guaranteed to make progress over an arbitrary period of time, regardless of the timing/ordering of thread execution; and so we can say that the threads finish independently. All wait-free programs are lock-free.

## Lock-freedom without garbage collection
> [aturon](https://aturon.github.io/blog/2015/08/27/epoch/)

In order to use and mutate data across threads, synchronization is necessary. The naive solution is to wrap the data in a `Mutex` to lock access. 

However, `Mutex` requires significant between threads accessing the data even if they are accessing disjoint pieces of the data. In this scenario, writes to update the lock state lead to a large amount of cache invalidation traffic. 

*Also, concurrent programming risks [deadlocks](https://en.wikipedia.org/wiki/Deadlock) and [priority inversion](https://en.wikipedia.org/wiki/Priority_inversion)*

**A more radical alternative is lock-free data structures, which use atomic operations to make direct changes to the data structure without further synchronization. They are often faster, more scalable, and more robust than lock-based designs.**

## Epoch-Based Reclamation
> [aturon post](https://aturon.github.io/blog/2015/08/27/epoch/)

There are a few non-GC-based ways of managing memory for lock-free code, but they all come down to the same core observations:
1. There are two sources of reachability at play – the data structure, and the snapshots in threads accessing it. Before we delete a node, we need to know that it cannot be reached in either of these ways.
2. Once a node has been unlinked from the data structure, no new snapshots reaching it will be created.

* [Practical Lock-Freedom](https://www.cl.cam.ac.uk/techreports/UCAM-CL-TR-579.pdf) by Keir Fraser

The epoch scheme works by having
1. a global epoch counter (taking on values 0, 1 and 2)
2. a global list of garbage for every epoch
3. an "active" flag for each thread
4. an epoch counter for each thread

When a thread wants to perform an operation on the data structure, it first sets its “active” flag, and then updates its local epoch to match the global one. If the thread removes a node from the data structure, it adds that node to the garbage list for the current global epoch. (Note: it’s very important that the garbage go into the current global epoch, not the previous local snapshot.) When it completes its operation, it clears the “active” flag.

To try to collect the garbage (which can be done at any point), a thread walks over the flags for all participating threads, and checks whether all active threads are in the current epoch. If so, it can attempt to increment the global epoch (modulo 3). If the increment succeeds, the garbage from two epochs ago can be freed.

Why do we need three epochs? Because “garbage collection” is done concurrently, it’s possible for threads to be in one of two epochs at any time (the “old” one, and the “new” one). But because we check that all active threads are in the old epoch before incrementing it, we are guaranteed that no active threads are in the third epoch.

## ReadingQ

* [1024cores - Introduction to Lock-Free Algorithms](http://www.1024cores.net/home/lock-free-algorithms)
* [Crossbeam Metalink](https://github.com/crossbeam-rs/rfcs/wiki)
* **[Rust New Channels Announcement](https://stjepang.github.io/2019/03/02/new-channels.html)** (3/2/19)
* [Lock Free and Wait Free Definitions by Concurrency Freaks](https://concurrencyfreaks.blogspot.com/2013/05/lock-free-and-wait-free-definition-and.html)
* [Exploring Lock Free Rust Part I](https://morestina.net/blog/742/exploring-lock-free-rust-1-locks)
* [aturon introducing Crossbeam](https://aturon.github.io/blog/2015/08/27/epoch/)
* [Designing Channels](https://stjepang.github.io/2017/08/13/designing-a-channel.html)
* [Bus-queueL Lock-free Bounded Non-Blocking Pub-Sub Queue](https://github.com/filipdulic/bus-queue)
    * `parking_lot`, `hash-brown` (soon to be added to `rust::std `)
    * [blog post](http://www.rossbencina.com/code/lockfree)
    * [jonhoo/bus](https://github.com/jonhoo/bus)
* [Rust concurrency checker](https://github.com/carllerche/loom)

## Papers
* [Obstruction-Free Synchronization: Double-Ended Queues as an Example](http://cs.brown.edu/~mph/HerlihyLM03/main.pdf)
* [A Methodology for Creating Fast Wait-Free Data Structures](http://www.cs.technion.ac.il/~erez/Papers/wf-methodology-ppopp12.pdf)

### Videos
* [Lock-Free Programming (or, Juggling Razor Blades), Part I](https://www.youtube.com/watch?v=c1gO9aB9nbs)
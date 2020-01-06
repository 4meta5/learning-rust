# Lock-Free Notes
> **[Fear and Loathing in Lock-Free Programming](https://medium.com/@tylerneely/fear-and-loathing-in-lock-free-programming-7158b1cdd50c)**

**Atomicity** conveys an indivisible, uninterruptible operation that will either 100% succeed or 100% fail.

Several CPU architectures allow the conditional setting of an atomic value if the current value is known. This is often referred to as *Compare and Swap* CAS (also "Compare and Set" sometimes). The hardware enforces the variant that only one thread "wins" if several threads attempt a CAS at the same time.

**Spinlock**: Spin in a loop until we can successfully change a lock variable from unlocked to locked. Only one thread will be granted exclusive access at a time! 

In addition to spinlocks avoiding the kernel scheduling overhead of going to sleep and waking up again, we also avoid CPU frequency scaling slowdowns that can happen while blocked on a mutex, so when we enter our critical section we don’t need to pay a start-up tax.

**Traditional mutexes** put a thread to sleep when they block, increasing the minimum latency for acquiring them from another thread. Modern mutexes are often a hybrid approach between a **spinlock** and a traditional mutex. **Hybrid mutexes** will attempt to acquire the lock quickly in userspace using atomic operations, without giving away their kernel-scheduled appointment on the CPU core. If the gambit didn’t pan out, the hybrid mutex will put the thread to sleep with a blocking syscall.

*Spinlock vs Mutex*: Choose spinlock if you know the expected time to acquire the lock is less expensive to you than the overhead of a modern hybrid mutex, or if you want a piece of code to be extremely responsive at the cost of some other system resource like power. But as the expected time to acquire the lock goes up, the balance shifts in favor of the mutex (mutexes in most popular threaded languages use the hybrid approach today).

## Treiber Stack

Looks like a linked list, with a head that points to the tip and nodes that point to the next node. We can push things into a lock-free stack by
1. create a node (effectively the same as a linked list)
2. read the current `stack.head` and set our `node.next` to it
3. CAS the stack's head from the head to our new node. If it worked, done; if not, GOTO step (2)

Popping the stack is similar:
1. read the current `stack.head`. If it's not set, either retry or return nothing if you have blocking or non-blocking semantics.
2. if head is set, try to pop it; attempt to CAS the `head` to the `head.next` value. If successful, return the severed head; otherwise GOTO step (1)

If another thread either pushed or popped a node after we read the head value but before we performed our CAS, then our CAS will fail because the current value is no longer what we read before.

This is a common theme in lock-free algorithms: spin until successful. This means that if a system has high contention (threads competing for the same resource), or spends lots of time doing things that look like spinlocks, a lock-free algorithm could be far less efficient with CPU resources than a mutex that puts blocked threads to sleep. If lots of threads are looping and throwing away work that they do, another approach may work better.

## Wait-Free Algorithms

**Wait-free** algorithms are a subset of lock-free algorithms that guarantee bounded time execution (atomic variables `+` bounded number of steps `~=>` wait-free algorithm).

*[Incrementing/Decrementing an Atomic Reference Counter](http://www.1024cores.net/home/lock-free-algorithms/introduction)* => what Python, Swift, and sometimes Rust use for keeping track of objects shared by multiple threads that need to be destroyed exactly once when all threads are done. This is a simple form of garbage collection!

**Wait-free Population Oblivious**: the number of steps we take in our code is not dependent on the number of threads participating because there is a constant number of instructions

## Shadow Paging

Sometimes we want to atomically update multiple items stored in a tree structure. The basic idea is:
1. Read the pointer to the root
2. Copy the things we want to change into new tree nodes, then go up the tree creating new (copied) nodes that reference the previous copied and changed level, going up the tree until we reach the root. All of this is done without changing the original tree.
3. Finally, we CAS the root to point to our changed pages.
4. If the CAS worked, our multi-item transaction was successful. if not, we either retry or propagate an error to the next higher level of our system.

This copy-on-write technique is useful, but can involve excessive copying. In practice, read-writer locks often outperform.

## Examples of Real-World Algorithms for Distributed Systems
* [Transactions in MongoDB, Cassandra, Zookeeper and Others...if we couldn't implement atomic writes ourselves](http://rystsov.info/2012/09/01/cas.html)
* [From CD to ACID: Adding Atomicity and Isolation to DynamoDB](https://github.com/awslabs/dynamodb-transactions/blob/master/DESIGN.md)


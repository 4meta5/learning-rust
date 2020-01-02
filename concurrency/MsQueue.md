# Michael Scott Queue

*Blocking algorithms* allow a slow or delayed process to prevent faster processes from completing operations on the shared data structure indefinitely.

*Non-blocking algorithms* guarantee that if there are one or more active processes trying to perform operations on a shared data structure, some operation will complete within a finite number of time steps.
* on asynchronous multiprocessor systems, blocking algorithms suffer significant performance degradation when a process is halted or delayed at an inopportune moment
* sources of degradation: processor scheduling preeemption, page faults, and cache misses

**Wait-free*** algorithms are non-blocking and starvation free; they guarantee that every active process will make progress within a bounded number of time steps

**Linearizable**: an implementation of a data structure is linearizable if it can always give an external oberserver, observing only the abstract data structure operations, the illusion that each of these operations takes effect instantaneously at some point between its invocation and its response.

## Lock-Free Algorithms for Concurrent FIFO queues

`compare_and_swap`: takes as arguments the address of a shared memory location, an expected value, and a new value. If the shared location currently holds the expected value, it is assigned the new value atomically. A Boolean return value indicates whether the replacement occurred

`compare_and_swap` algorithms must deal with the **ABA problem**: if a process reads a value `A` in a shared location, computes a new value, and then attempts a `compare_and_swap` operation, the `compare_and_swap` may succeed when it should not, if between the read and the `compare_and_swap` some other process(es) change the `A` to `B` and back to `A` again
* *Solution*: associate a modification counter with a pointer, to always access the counter with the pointer in any read-modify-`compare_and_swap`

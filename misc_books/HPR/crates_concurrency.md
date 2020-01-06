# Multithreading Crates

**Crossbeam** gives us multithreaded queues and stacks that allow us to insert data and consume data from different threads. We can coordinate initial processing by some threads and processing by a second group of threads. A simple example,

```rust
extern crate crossbeam;

use std::thread;
use std::sync::Arc;

use crossbeam::sync::MsQueue;

fn main() {
    let queue = Arc::new(MsQueue::new());

    let handles: Vec<_> = (1..6)
        .map(|_| {
            let t_queue = queue.clone();
            thread::spawn(move || {
                for _ in 0..1_000_000 {
                    t_queue.push(10);
                }
            })
        })
        .collect();
    
    for handle in handles {
        handle.join().unwrap();
    }

    let final_queue = Arc::try_unwrap(queue).unwrap();
    let mut sum = 0:
    while let Some(i) = final_queue.try_pop() {
        sum += i;
    }

    println!("Final sum: {}", sum);
}
```

We still needed to use an `Arc` to control the multiple references to the queue. This is necessary because the queue itself cannot be duplicated and shared (it has no reference count).

Crossbeam also gives us LIFO stacks. 

```rust
extern crate crossbeam;

use std::thread;
use std::sync::Arc;
use std::time::Duration;

use crossbeam::sync::{MsQueue, TreiberStack};

fn main() {
    let queue = Arc::new(MsQueue::new());
    let stack = Arc::new(TreiberStack::new());

    let in_queue = queue.clone();
    let in_stack = stack.clone();
    let in_handle = thread::spawn(move || {
        for i in 0..5 {
            in_queue.push(i);
            in_stack.push(i);
            println!("Pushed :D");
            thread::sleep(Duration::from_millis(50));
        }
    });

    let mut final_queue = Vec::new();
    let mut final_stack = Vec::new();

    let mut last_q_failed = 0;
    let mut last_s_failed = 0;

    loop {
        // Get the queue
        match queue.try_pop() {
            Some(i) => {
                final_queue.push(i);
                last_q_failed = 0;
                println!("Something in the queue! :)");
            }
            None => {
                println!("Nothing in the queue :(");
                last_q_failed += 1;
            }
        }

        // Get the stack
        match stack.try_pop() {
            Some(i) => {
                final_stack.push(i);
                last_s_failed = 0;
                println!("Something in the stack! :)");
            }
            None => {
                println!("Nothing in the stack :(");
                last_s_failed += 1;
            }
        }

        // Check if we finished
        if last_q_failed > 1 && last_s_failed > 1 {
            break;
        } else if last_q_failed > 0 || last_s_failed > 0 {
            thread::sleep(Duration::from_millis(100));
        }
    }

    in_handle.join().unwrap();

    println!("Queue: {:?}", final_queue);
    println!("Stack: {:?}", final_stack);
}
```

If we used `pop()` instead of `try_pop()`, it will block the thread if the queue or the stack is empty. This will happen in any case once all values get popped, since no new values are being added, so the `try_pop()` method will thereby help not block the main thread and end gracefully.
* (in the code above) it checks whether all the values were popped by counting how many times it failed to pop a new value

## Scoped Threads

Standard library threads have their own stack, so if we want to use variabled created in the main thread, we need to **send** them to the thread.
* having their owns stack also means that they will also consume more memory and eventually make the system slower.

Scoped threads are special threads provided by Crossbeam that allow sharing stacks between them. For use,
1. create a `Scope` by calling `crossbeam::scope()`
2. pass a closure that receives the Scope
3. call `spawn()` in that scope

Now, you can share immutable variables among threads if they were created inside the scope or moved to it.

```rust
extern crate crossbeam;

fn main() {
    let all_nums: Vec<_> = (0..1_000_u64).into_iter().collect();
    let mut results = Vec::new();

    crossbeam::scope(|scope| {
        for num in &all_nums {
            results.push(scope.spawn(move || num * num + num * 5 + 250))
        }
    });

    let final_result: u64 = results.into_iter().map(|res| res.join()).sum();
    println!("Final result: {}", final_result);
}
```

The `Threadpool` crate enables you to iterate over all work and, for each of the small chunks, you may call something similar to `thread::spawn()`. Each task is assigned an idle thread and no new thread will be created per task. 

> **reminder**: the ideal number of threads we should spawn to do all the work should be around the number of virtual processors in the system

```rust
extern crate num_cpus;
extern crate threadpool;

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use threadpool::ThreadPool;

fn main() {
    let pool = ThreadPool::with_name("my worker".to_owned(), num_cpus::get());
    println!("Pool threads: {}", pool.max_count());

    let result = Arc::new(AtomicUsize::new(0));

    for i in 0..1_0000_000 {
        let t_result = result.clone();
        pool.execute(move || {
            t_result.fetch_add(i, Ordering::Relaxed);
        });
    }

    pool.join();

    let final_res = Arc::try_unwrap(result).unwrap().into_inner();
    println!("Final result: {}", final_res);
}
```

## Rayon Parallel Iterators

Rayon uses a parallel iteration technique called **work stealing**. For each iteration of the parallel iterator, the new value or values get added to a queue of pending work. When a thread finishes its work, it checks whether there is any pending work to do and, if there is, it starts processing it. 

```rust
extern crate rayon;

use rayon::prelude::*;

fn main() {
    let result = (0..1_000_000_u64)
        .into_par_iter()
        .map(|e| e * 2)
        .sum::<u64>();

    println!("Result: {}", result);
}
```

Sometimes parallelizing doesn't automatically improve its performance. Take into account that if you need to update some shared information between the threads, they will need to synchronize somehow, and you will lose performance. Therefore, **multithreading is only great if workloads are completely independent and you can execute one without any dependency on the rest.**
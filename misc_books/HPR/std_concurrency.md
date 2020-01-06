# Concurrency with Standard Library

*Data Race Example* => Imagine we are programming a request counter. If we separate the load between two processors, and each processor receives a request, the shared counter should go up by two, right? Each thread wants to add 1 to the counter. For that, they load the current counter in the CPU, they add one to it and then save it again in the RAM. This takes some time, especially loading it from RAM, which means that if they both load the counter at the same time, they will both have the current counter in the CPU. If both add one to the counter and save it back, the value in the RAM will only add one request, instead of two, because both processors will save the new +1 value in the RAM.

Neither `Sync` nor `Send` add methods to a given type. Once compiled, they will not occupy any memory. Instead, they really are just *marker* traits in that they tell the compiler if multithreading is safe.

**`T` is `Sync` means that the reference `&T` (`Send`) can be shared between threads.**
* `Send` types can be moved between threads, `Sync` can be "shared" (the reference, which is send can be shared) between threads
* Most types are `Send`, such as `u32` for example
* raw pointers can't be shared between threads (dereferencing raw pointers is unsafe)

`Rc<T>` isn't meant to be shared between threads (that's why we have atomic types like `Arc<T>`)
* `Rc<T>` uses `std::cell::Cell` to achieve interior mutability on the counter that increases for every clone. So even if we have an immutable `Rc<T>`, the counter still increases on every clone because of how it uses `std::cell::Cell`
* `Rc` uses a `Cell` to store a count of references so that you can call the `clone()` method on an immutable `Rc` and still update the count of references

* if you don't need to send a reference to another thread, use `Rc` instead of `Arc` because `Arc` requires more checks because the counter increments are atomic

`Cell` implements interior mutability by moving values in and out of the `Cell`. If the value is a `Copy` type, then it can be inserted into the `Cell` or read from within the `Cell`; however this doesn't hold for more complex types like `Vec<T>` or `HashMap<T>`

`RefCell` allows you to get mutable references to the value it wraps if there are no other references to it. This is unsafe so it uses a flag to tell `RefCell` whether or not it's currently borrowed or not. 
* If borrowed for read => more read-only borrows can be generated via the `borrow()` method, but no mutable borrow can be done
* if mutably borrowed with `borrow_mut()`, you will not be able to borrow it mutably or immutably
* `borrow` and `borrow_mut` check the current borrow status at runtime, not compile time
* non-panicking alternatives are `try_borrow` and `try_borrow_mut()`

Since the borrow flag is not thread safe, `RefCell` will not be `Sync`. 

## Multithreading

**Multithreading**: each thread has access to shared memory and creates its own stacks so that it can work independently

* ideally you have about the same number of threads working at the same time as the number of virtual CPUs in your PC/server
* usually number of threads is twice the number of CPU cores because of hyperthreading (in which one core can run two threads at the same time by using its won hardware scheduler to decide which parts of each thread run at a given point in time)

* if you don't put a limit on the number of threads running and run too many of them, because of all the satcks that need to be created per thread, you will consume a lot of RAM
* it is not uncommon for some web servers to create one thread per request; this makes things much slower when the load is high because it requires a lot of RAM

**Asynchronous** programming -- possibility for one thread to run multiple I/O requests while not blocking the actual thread
* if the thread goes idle, it will sleep for some time and then poll for new requests; the OS will wake the thread up when there is new information for it...this approach uses the minimum possible resources for I/O operations

*Vectorization*: uses special CPU instructions and registers where you can enter more than one variable and perform the same operation in all of them at the same time.
* special instructions for vectorization are called the **SIMD** (Single Instruction Multiple Data) family
* [Faster](https://crates.io/crates/faster) - SIMD for humans

### Creating Threads

`std::thread` with the `spawn()` function grants us the ability to receive a closure or a pointer to a function to execute it; it returns a handle to the thread thereby allowing us to manage it from the outside

```rust
use std::thread;

fn main() {
    println!("Before the thread!");

    let handle = thread::spawn(|| {
        println!("Inside the thread!");
    });
    println!("After thread spawn!");

    handle.join().expect("the thread panicked");
    println!("After everything!");
}
```

The `Inside the thread!` and `After thread spawn!` messages could be ordered in any way (although spawning the thread takes more time than printing in the screen buffer).

In some programming languages, the characters of both messages might get mixed and the final message could be an incomprehensible characters.
* Rust avoids this possibility because the standard output file descriptor is accessed with a `Mutex` => the `println!()` macro locks `stdout` while it writes the message and, if a new message wants to be written, it has to wait until the first write finishes.

The `join` method called in the thread handle makes the current thread wait for the other one to finish. `join()` returns a `Result` because it might have panicked before finishing.

**panic is something that happens at the thread level** => if the main thread panics, the whole program exist, but if a non-main thread panics, you can recover from it.

In C/C++, when you exit a program, the memory gets handed back to the kernel and it ends. Rust makes sure that it calls all the destructors in the current stack such that all variables are dropped gracefully.
* **stack unwinding**
* pro: allows us to deallocate memory (ie close files, write logs, update databases, etc)
* con: each time we call `unwrap()` or `expect()`, a new branch -- either things go wrong and the thread panics or things go as they should -- if they panic, the compiler has to add the whole code for the stack unwinding (which makes the executables much bigger)

### Moving data between threads

1. share the binding between threads
2. send data between threads with `move` before the closure passed to `std::thread::Builder::new().spawn()`

`Arc` is a reference-counted pointer that can be shared between threads. `Arc` counts the references with an atomic counter `=>` the kernel will ensure that all updates to the reference count happen one by one, thereby making it thread-safe.

The `Clone` trait for `Arc` returns a new `Arc` and increases the reference count. Since both instances of `Arc` will have the same pointers to the reference counter and vector, we will effectively be sharing the vector.

A `Mutex` gets locked when you call `lock()` and gets unlocked when it goes out of scope...Sometimes, it might even be useful to create artificial scopes to unlock the `Mutex` as soon as possible if our work involves more than one line and we need a binding.

**`Mutex` poisoning**
Your thread panics while the Mutex is locked `=>` the `lock()` function in another thread returns `Result::Err(_)` so if we call `unwrap()` every time we `lock()` our `Mutex`, all threads would panic
* when a `Mutex` is poisoned because a thread panicked while having it locked, the error result of calling the `lock()` method will return the poisoning error
* recover by calling `into_inner()`

```rust
use std::thread;
use std::sync::{Arc, Mutex};
use std::time::Duration;

fn main() {
    let my_vec = Arc::new(Mutex::New(Vec::new()));

    let t_vec = my_vec.clone();
    let handle = thread::Builder::new()
        .name("my thread".to_owned())
        .spawn(move || {
            for i in 0..10 {
                let mut vec = t_vec.lock().unwrap();
                vec.push(i);
                panic!("Panicking the secondary thread");
            }
        })
        .expect("could not create the thread");

    thread::sleep(Duration::from_Secs(1));

    for i in 0..10 {
        let mut vec = match my_vec.lock() {
            Ok(g) => g,
            Err(e) => {
                println!("The secondary thread panicked, recovering...");
                e.into_inner()
            }
        };
        vec.push(i);
    }

    if handle.join().is_err() {
        println!("Something bad happened :(");
    }

    let vec_mutex = Arc::try_unwrap(my_vec).unwrap();
    let f_vec = match vec_mutex.into_inner() {
        Ok(g) => g,
        Err(e) => {
            println!("The secondary thread panicked, recovering...");
            e.into_inner()
        }
    };
    println!("Final vector: {:?}", f_vec);
}
```

Don't `unwrap()` a `Mutex` in a critical application as it will make all threads panic if you do it in all your threads after the first panic.

### Channels between threads

**channels**: multi-producer, single-consumer FIFO communication primitives
* multi-producer => can send the data to the receiver from multiple threads at the same time
* single-consumer => only one receiver will receive data from all the associated senders in the channel

One thread can manage the I/O interface with the communication or logging mechanism, while the others can send this thread the information they want to log or communicate. 

A channel consists of a `Sender` and a `Receiver`. `Sender` implements `Clone`, so that cloning the sender, multiple threads can send information to the associated receiver.

Two types of senders:
1. `Sender` -- send the message to receiver without checking anything extra
2. `SyncSender` -- send the message only when the receiver's buffer has enough space (blocks current thread until the message is sent)

Channels are created using `channel()` and `sync_channel()` functions in the `std::sync::mpsc` module. They return a tuple with a `Sender` or `SyncSender`, respectively as the first element and a `Receiver` as the second one.
* because `Sender` and `Receiver` implement `Send`, they can safely be sent to another thread with the `move` keyword
* for a synchronous channel, the `sync_channel()` will require a `usize` to set the buffer size; `Sender` blocks if the buffer is full
* asynchronous channels work as if they have an infinite buffer such that they will always accept sending new data

Each channel can only send or receive one particular type of data. You can configure to send your own types.

```rust
use std::thread;
use std::sync::mpsc::*;
use std::time::Duration;

fn main() {
    let (sender, receiver) = channel();

    let handles: Vec<_> = (1..6)
        .map(|i| {
            let t_sender = sender.clone();
            thread::Builder::new()
                .name(format!("sender-{}", i))
                .spawn(move || {
                    t_sender.send(
                        format!("Hello from sender {}!", i)
                    ).unwrap();
                })
                .expect("couldn't create the thread")
        })
        .collect();
    
    while let Ok(message) = receiver.recv_timeout(Duration::from_secs(1)) {
        println!("{}", message);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Finished");
}
```

The threads in the example above aren't executed in any particular order. They send the message to the receiver, and the receiver reads them in the received order. Since this is asynchronous, we do not need to wait for the receiver to empty the buffer to send new messages (so it's very lightweight). In fact, we could join the threads before reading any messages from the receiver.
#![feature(futures_api, pin, async_await, await_macro, arbitrary_self_types)]
extern crate futures;
extern crate libc;
#[macro_use]
extern crate log;
extern crate pretty_env_logger;
extern crate core;

use core::pin::Pin;

use futures::future::{Future, FutureObj};
use futures::task::{Spawn, SpawnError, LocalWaker};
use futures::task::Wake;
use futures::Poll;
use libc::{fd_set, select, timeval, FD_ISSET, FD_SET, FD_ZERO};

use std::os::unix::io::RawFd;

use std::cell::{Cell, RefCell};
use std::collections::{VecDeque, BTreeMap};
use std::rc::Rc;
use std::sync::Arc;

mod async_tcp_stream;
mod async_tcp_listener;

pub use crate::async_tcp_stream::AsyncTcpStream;
pub use crate::async_tcp_listener::AsyncTcpListener;

// reactor lives in a thread local variables...this is where all magic happens
thread_local!(
    static REACTOR: Rc<EventLoop> = Rc::new(EventLoop::new());
)

type TaskId = usize;

pub fn run<F: Future<Output = ()> + Send + 'static>(f: F) {
    REACTOR.with(|reactor| reactor.run(f))
}

pub fn spawn<F: Future<Output = ()> + Send + 'static>(f: F) {
    REACTOR.with(|reactor| reactor.spawn(f))
}

// Our waker token. It stores the index of the futures in the wait queue
#[derive(Debug)]
struct Token(usize);

impl Wake for Token {
    fn wake(arc_self: &Arc<Token>) {
        debug!("waking {:?}", arc_self);

        let Token(idx) = **arc_self;

        // get access to the token by way of TLS and call wake
        REACTOR.with(|reactor| {
            let wakeup = Wakeup {
                index: idx,
                waker: unsafe { futures::task::local_waker(arc_self.clone()) },
            };
            reactor.wake(wakeup);
        });
    }
}

// wakeup notification struct stores the index of the future in the wait queue
// and waker
struct Wakeup {
    index: usize,
    waker: LocalWaker,
}

// Task is a boxed future with Output = ()
struct Task {
    future: FutureObj<'static, ()>,
}

impl Task {
    // returning Ready will lead to task being removed from wait queues and dropped
    fn poll(&mut self, waker: LocalWaker) -> Poll<()> {
        let future = Pin::new(&mut self.future);

        match future.poll(&waker) {
            Poll::Ready(_) => {
                debug!("future done");
                Poll::Ready(())
            }
            Poll::Pending => {
                debug!("future not yet ready");
                Poll::Pending
            }
        }
    }
}

// The "real" event loop
struct EventLoop {
    read: RefCell<BTreeMap<RawFd, LocalWaker>>,
    write: RefCell<BTreeMap<RawFd, LocalWaker>>,
    counter: Cell<usize>,
    wait_queue: RefCell<BTreeMap<TaskId, Task>>,
    run_queue: RefCell<VecDeque<Wakeup>>,
}

impl EventLoop {
    fn new() -> Self {
        EventLoop {
            read: RefCell::new(BTreeMap::new()),
            write: RefCell::new(BTreeMap::new()),
            counter: Cell::new(0),
            wait_queue: RefCell::new(BTreeMap::new()),
            run_queue: RefCell::new(VecDeque::new()),
        }
    }

    pub fn handle(&self) -> Handle {
        REACTOR.with(|reactor| Handle(reactor.clone()))
    }

    // a future calls this to register its interest
    // in socket's "ready to be read" events
    fn add_read_interest(&self, fd: RawFd, waker: LocalWaker) {
        debug!("adding read interest for {}", fd);

        if !self.read.borrow().contains_key(&fd) {
            self.read.borrow_mut().insert(fd, waker);
        }
    }

    fn remove_read_interest(&self, fd: RawFd) {
        debug!("removing read interest for {}", fd);

        self.read.borrow_mut().remove(&fd);
    }

    // see above
    fn remove_write_interest(&self, fd: RawFd) {
        debug!("removing write interest for {}", fd);

        self.write.borrow_mut().remove(&fd);
    }

    fn add_write_interest(&self, fd: RawFd, waker: LocalWaker) {
        debug!("adding write interest for {}", fd);

        if !self.write.borrow().contains_key(&fd) {
            self.write.borrow_mut().insert(fd, waker);
        }
    }

    // waker calls this to put the future on the run queue
    fn wake(&self, wakeup: Wakeup) {
        self.run_queue.borrow_mut().push_back(wakeup);
    }

    fn next_task(&self) -> (TaskId, LocalWaker) {
        let counter = self.counter.get();
        let w = Arc::new(Token(counter));
        self.counter.set(counter + 1);
        (counter, unsafe { futures::task::local_waker(w) })
    }

    // create a task, poll it once and push it on wait queue
    fn do_spawn<F: Future<Output = ()> + Send + 'static>(&self, f: F) {
        let (id, waker) = self.next_task();
        let f = Box::new(f);
        let mut task = Task {
            future: FutureObj::new(f),
        };

        {
            // if the task is ready immediately, don't add it to wait_queue
            if let Poll::Ready(_) = task.poll(waker) {
                return;
            }
        };

        self.wait_queue.borrow_mut().insert(id, task);
    }

    // the meat of the event loop
    // we're using select(2) because it's simple and it's portable
    pub fn run<F: Future<Output = ()> + Send + 'static>(&self, f: F) {
        self.do_spawn(f);

        loop {
            debug!("select loop start");

            // event loop iteration timeout. if no descriptor
            // is ready we continue iterating
            let mut tv: timeval = timeval {
                tv_sec: 1,
                tv_usec: 0,
            };

            // initialize fd_sets (file descriptor sets)
            let mut read_fds: fd_set = unsafe { std::mem::zeroed() };
            let mut write_fds: fd_set = unsafe { std::mem::zeroed() };

            unsafe { FD_ZERO(&mut read_fds) };
            unsafe { FD_ZERO(&mut write_fds) };

            let mut nfds = 0;

            // add read interests to read fd_sets
            for fd in self.read.borrow().keys() {
                debug!("added fd {} for read", fd);
                unsafe { FD_SET(*fd, &mut read_fds as *mut fd_set) };
                nfds = std::cmp::max(nfds, fd + 1);
            }
        }
    }
}
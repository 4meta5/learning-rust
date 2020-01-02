# Notes from the Programming Servo Series
> *[gterzian's medium](https://medium.com/@polyglot_factotum)*

**Channels, and the loops receiving messages on them, combine multi-threading with iteration. Iteration is by nature sequential, making it easier to reason about the behavior of your concurrent system.**

*An event-loop is quite simply a loop, in a single thread, that will poll sources of events at each iteration, and handle them sequentially...the essence of an event-loop is that it will poll something, or a few different things, and handle 'events` coming from these sources, at each iteration. Using that technique, your event-loop can become the driving force of a complicated multi-threaded system.* - [event loop](https://medium.com/programming-servo/programming-servo-the-script-event-loop-be687b985b3e)

## Relevant Blog Posts
* [Programming Servo: A background hang-monitor](https://medium.com/programming-servo/programming-servo-a-background-hang-monitor-73e89185ce1)
* [Rust concurrency patterns: Still communicating by moving senders (like it's 2018)](https://medium.com/@polyglot_factotum/rust-concurrency-patterns-communicate-by-sharing-your-sender-re-visited-9d42e6dfecfa)
* [Programming Servo: Zen and the art of removing blocks from your system](https://medium.com/@polyglot_factotum/programming-servo-zen-and-the-art-of-removing-blocks-from-your-system-51c1b7d404e3)

## Security

* **[Loophole: Timing Attacks on Shared Event Loops in Chrome](https://arxiv.org/abs/1702.06764)**
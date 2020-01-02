# Event Loop

**JavaScript Event Loop**
Each `thread` gets its own **event loop**, so each web worker gets its own, so it can execute independently.

**Event Loop**: a loop to handle events, preferrably one at a time

The **event loop** runs continually, executing any tasks queued. An event loop has multiple task sources which guarantees execution order within that source, but the browser gets to pick which source to take a task from on each turn of the loop. This allows the browser to give preference to performance sensitive tasks such as user-input.

**Tasks** execute in order and the browser may render between them. **Microtasks** execute in order and are executed after every callback, as long as no other JavaScript is mid-execution at the end of each task.

## Organizing Code (and the Actor Model)

Try to write down a clear “processing model” for each of your concurrent components. That means a list of steps that will happen at each “turn”, in precise order. If you can’t do that, you’ve got **concurrent spaghetti** on your hands, and that has likely to do with too much sharing of state among these components.

How to build layered cakes instead? Modelling each component as a separate event-loop, is a good start. An event-loop implies that only a single “task” or “event” will be handled at each iteration of the loop. It also implies the event-loop has some state that is local only and not shared with other event-loops. The way to “communicate” between event-loops is by enqueuing task on each others queues(some people refer to this as the “Actor model”, or at least a variant thereof).


* [Communicate by Sharing Your Sender](https://medium.com/@polyglot_factotum/rust-concurrency-patterns-communicate-by-sharing-your-sender-11a496ce7791)
* [Tasks, microtasks, queues and schedules](https://jakearchibald.com/2015/tasks-microtasks-queues-and-schedules/)

* [Futures Aware Channel (I see a lot recently)](https://docs.rs/futures/0.1.26/futures/sync/mpsc/index.html)
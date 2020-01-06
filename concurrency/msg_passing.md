# Message Passing: Synchronous Rendezvous

Synchronous rendezvous involves one-way transfer of data from sender to receiver. The sender and receiver must both be “ready” for the transfer to occur, and both proceed independently afterwards. Although no data is explicitly passed from receiver back to the sender, the rendezvous itself provides synchronization information to both the sender and receiver.

Often synchronous communication is described in terms of “channels” connecting senders and receivers. Sometimes these channels have a buffering capacity which allows some number of messages to be stored for later delivery. If the buffer is full, senders will be blocked waiting for capacity. If the buffer is empty, receivers will be blocked waiting for messages. A buffer size of zero (no buffering) will block a sender/receiver until a corresponding receiver/sender is available.

## Resources
* [Message Passing, part 1: Synchronous Rendezvous](http://www.dalnefre.com/wp/2010/07/message-passing-part-1-synchronous-rendezvous/comment-page-1/)
* [Message Passing, part 2 – Object-Oriented Method Invocation](http://www.dalnefre.com/wp/2010/07/message-passing-part-2-object-oriented-method-invocation/)
* [High Availability for Mutable Shared State](http://www.dalnefre.com/wp/2011/11/high-availability-for-mutable-shared-state/#queue)
* [Single Writer Principle](https://mechanical-sympathy.blogspot.com/2011/09/single-writer-principle.html)
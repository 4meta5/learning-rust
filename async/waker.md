# Waker API

The Waker API is how the executor and event source coordinate waiting and waking. When the Future is polled, the executor passes it a waker, which an event source will register and eventually wake.

## References

* [WithoutBlogs: Waker API Part I](https://boats.gitlab.io/blog/post/wakers-i/)
    * [II (Across Threads)](https://boats.gitlab.io/blog/post/wakers-ii/)
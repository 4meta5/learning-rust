# Pinning

**Pinned references** are pointers that *pin* the data that they refer to in a particular memory location to guarantee that the object will never move again. 

> The pinning API was motivated by [self-referential structs](https://boats.gitlab.io/blog/post/2018-01-25-async-i-self-referential-structs/) and intrusive lists

*TODO*: Understand why pinning is a necessary addition in order to enable memory-safe self-referential structs

## ReadingQ

* [Rethinking Pin](https://boats.gitlab.io/blog/post/rethinking-pin/)
* [A Formal Look at Pinning](https://www.ralfj.de/blog/2018/04/05/a-formal-look-at-pinning.html)
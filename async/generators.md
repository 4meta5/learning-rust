# Generators
> notes on generators, async & await

Generators in Rust enable you to create functions that can "yield" (maintain their state such that they can be called again until they finally return). Here's a function that returns a generator:

```rust
#![feature(generators, generator_trait, conservative_impl_trait)]

use std::ops::Generator;

pub fn up_to(limit: u64) -> impl Generator<Yield = u64, Return = u64> {
    move || {
        for x in 0..limit {
            yield x;
        }
        return limit;
    }
}
```

This generator yields every number from 0 to the limit (exclusive) and then returns the limit.

Generators have two use cases:
* **Iterators**: A generator which yields `T` and returns `()` can be treated as an Iterator of `T`
* **Futures**: A generator which yields `()` and returns `Result<T, E>` can be considered as a Future of `T` and `E`.

> The main existing problem is that borrows cannot be allowed across yield points `=>` you cannot have a borrow for which the lifetime includes an `await`; a conceptual example that @withoutboats uses is a a self-referential struct

To better understand the generator problem, consider the internal representation.

A generator essentially creates an anonymous enum type; each of its variants is a state that it could be in at a particular yield point. For all of its variants, the generator saves all of the variables that it needs to continue to keep working once it is resumed.

Likewise, generators save a minimal representation of their "stack" when they yield. Rather than maintaining an entire statically sized stack for each generator, the generator's size is the size of the largest amount of state it could need to preserve at any of its yield points.

> The problem arises when some of that stack state that is being preserved references other items that you're preserving from the stack. Because all of this state is stored together in the "generator enum", this becomes a special case of self-referential structs.

## Practical Costs of Immovability are Marginal

The *generator problem* is that you can't move self-referential generators once you start calling `resume`. For iterators, this corresponds to the `next` method, whereas for futures, it corresponds to `poll`. 

> Until you start iterating or polling, the generator can be moved as much as you like, because it hasn't started running yet.

By convention and API design, you are unlikely to want to move a self-referential generator:
1. while you're building an iterator or future using combinators, you can move as much as you may like
2. Once you've started processing the generator, it's not normal to need to move it...
3. If you do need to be able to move it, putting it into the heap works

### Random Stuff

**`rustc`**: 
* parse
* typecheck
* borrowcheck
* lower to LLVM's representation

### References and Sources
* [WithoutBlogs: Async/Await I Self-Referential Structs](https://boats.gitlab.io/blog/post/2018-01-25-async-i-self-referential-structs/)
    * [II](https://boats.gitlab.io/blog/post/2018-01-30-async-ii-narrowing-the-scope/)
    * [IV](https://boats.gitlab.io/blog/post/2018-02-07-async-iv-an-even-better-proposal/)
    * [V](https://boats.gitlab.io/blog/post/2018-02-08-async-v-getting-back-to-the-futures/)

* [WithoutBlogs: Generator I](https://boats.gitlab.io/blog/post/generators-i/)
    * [II](https://boats.gitlab.io/blog/post/generators-ii/)
# Practice
> Some practice code patterns that I found(/find) useful when learning Rust

* Linked List -- `linkedlist.rs`
* Macros Pratice -- `recurrence.rs`
* Conversion for practicing `From` and `Into` -- `conversion.rs`
* State Machine -- `statemachine.rs`
* Combinators (`map`, `and_then`) -- `combinators.rs`

**2018 Rust AoC Solutions**
* [Diggsey/aoc2018](https://github.com/Diggsey/aoc2018)
* [BurntSushi/advent-of-code](https://github.com/BurntSushi/advent-of-code)
* [A Rusty Advent of Code](https://cprimozic.net/blog/a-rusty-aoc/) -- really solid walkthrough by Casey Primozic

* [Kuhn-Munkres Algorithm Implementation](https://github.com/nwtnni/hungarian) -- I really like this algorithm and someone implemented it in simplified form in Rust!

## Todo
* parsers and lexers
    * [read this first and then do the tutorial](http://lalrpop.github.io/lalrpop/crash_course.html)
    * [`syn`](https://github.com/dtolnay/syn)
    * [`quote`](https://github.com/dtolnay/quote)
    * [`combine`](https://github.com/Marwes/combine)
    * [`nom`](https://github.com/Geal/nom)
    * [`structopt`](https://crates.io/crates/structopt) for argument parsing
    * [ds_store parser](https://github.com/sinistersnare/ds_store/blob/master/README.md)
    * [Making Rust Float Parsing Fast and Correct](https://www.reddit.com/r/rust/comments/a6j5j1/making_rust_float_parsing_fast_and_correct/?st=JPQ2J3ZW&sh=cb57fb7f)
* lock-free and wait-free algorithms
    * `parking_lot`, `hash-brown`
    * [blog post](http://www.rossbencina.com/code/lockfree)
    * [jonhoo/bus](https://github.com/jonhoo/bus)
* building a domain specific language
    * check out Zokrates
    * [how to roll out custom DSL w/o getting hurt](https://www.slideshare.net/RReverser/building-fast-interpreters-in-rust)

## Code Patterns
* [Rust Flow](https://myrrlyn.net/blog/misc/rust-flow)

> great Rust blog for borrowing

* [Currying in Rust: Part 1](https://hashnode.com/post/currying-in-rust-cjpfb0i2z00cm56s2aideuo4z)
* [Currying in Rust: Part 2](https://hashnode.com/post/currying-in-rust-part-2-a-glimpse-of-generics-cjphbgun90025pms241ggh3d9)
* [Currying in Rust: Part 3](https://hashnode.com/post/currying-in-rust-part-3-the-circle-of-life-aka-why-borrowchecker-why-cjq3z1dd800dknds1sls4dqav)

* [Stacked Borrows Implemented](https://www.ralfj.de/blog/2018/11/16/stacked-borrows-implementation.html)
* [Barriers and Two-phase Borrows in Stacked Borrows](https://www.ralfj.de/blog/2018/12/26/stacked-borrows-barriers.html)

* [Rust API Guidelines](https://rust-lang-nursery.github.io/api-guidelines/about.html)
    * [Sealed Traits Code Pattern](https://rust-lang-nursery.github.io/api-guidelines/future-proofing.html)
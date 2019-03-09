# Practice

*Basics*
* [`linkedlist.rs`](./simple/src/linkedlist.rs): very simple linked list implementation to demonstrate the `struct`-`impl` code pattern common in Rust
* [`recurrence.rs`](./simple/src/recurrence.rs): basic introduction to Macros in Rust with recursion via the fibonacci sequence
* [`conversion.rs`](./simple/src/conversion.rs): practicing `From` and `Into` for conversion between types
* [`statemachine.rs`](./simple/src/statemachine.rs): Rust state machine pattern by [Hoverbear](https://hoverbear.org/2016/10/12/rust-state-machine-pattern/)
* [`combinators.rs`](./simple/src/combinator.rs): basic syntax and patterns for combinators (`map`, `and_then`)

## Code Pattern Resources <a name = "pattern"></a>
> **best resources**: [Rust Anthology Master List](https://github.com/brson/rust-anthology/blob/master/master-list.md), [Rust Patterns](https://crates.io/categories/rust-patterns)

* [Tricks](#tricks)
* [2018 AoC Solutions](#aoc)
* [Algorithms](#algo)
* [Parsing and Lexing](#parselex)
* [Currying](#curry)

### Tricks <a name = "tricks"></a>

* [Rust Tips by Spacejam](https://github.com/spacejam/elements-of-rust)
* [Rust Flow](https://myrrlyn.net/blog/misc/rust-flow)
* [Vorner's Rust Hacks](https://vorner.github.io/2019/02/03/hacks.html)

* [Rust API Guidelines](https://rust-lang-nursery.github.io/api-guidelines/about.html)
    * [Sealed Traits Code Pattern](https://rust-lang-nursery.github.io/api-guidelines/future-proofing.html)

* [Example of Interaction with IPFS in Rust](https://github.com/kpcyrd/ipfs.ink)

### 108 Advent of Code Solutions <a name = "aoc"></a>
* [Diggsey/aoc2018](https://github.com/Diggsey/aoc2018)
* [BurntSushi/advent-of-code](https://github.com/BurntSushi/advent-of-code)
* [A Rusty Advent of Code](https://cprimozic.net/blog/a-rusty-aoc/) -- really solid walkthrough by Casey Primozic

### Algorithms <a name = "algo"></a>

* [Kuhn-Munkres (hungarian) Algorithm Implementation](https://github.com/nwtnni/hungarian) -- I really like this algorithm and someone implemented it in simplified form in Rust!

### Parsing and Lexing <a name = "parselex"></a>

> *Lexers are used to recognize "words" that make up language elements, because the structure of such words is generally simple. Regular expressions are extremely good at handling this simpler structure, and there are very high-performance regular-expression matching engines used to implement lexers. Parsers are used to recognize "structure" of a language phrases. Such structure is generally far beyond what "regular expressions" can recognize, so one needs "context sensitive" parsers to extract such structure. Context-sensitive parsers are hard to build, so the engineering compromise is to use "context-free" grammars and add hacks to the parsers ("symbol tables", etc.) to handle the context-sensitive part. Neither lexing nor parsing technology is likely to go away soon. They may be unified by deciding to use "parsing" technology to recognize "words", as is currently explored by so-called scannerless GLR parsers. That has a runtime cost, as you are applying more general machinery to what is often a problem that doesn't need it, and usually you pay for that in overhead. Where you have lots of free cycles, that overhead may not matter. If you process a lot of text, then the overhead does matter and classical regular expression parsers will continue to be used.* - [stackoverflow](https://stackoverflow.com/a/2852716)

* [read this first and then do the tutorial](http://lalrpop.github.io/lalrpop/crash_course.html)
* [`syn`](https://github.com/dtolnay/syn)
* [`quote`](https://github.com/dtolnay/quote)
* [`combine`](https://github.com/Marwes/combine)
* [`nom`](https://github.com/Geal/nom)
* [`structopt`](https://crates.io/crates/structopt) for argument parsing
* [ds_store parser](https://github.com/sinistersnare/ds_store/blob/master/README.md)
* [Making Rust Float Parsing Fast and Correct](https://www.reddit.com/r/rust/comments/a6j5j1/making_rust_float_parsing_fast_and_correct/?st=JPQ2J3ZW&sh=cb57fb7f)
* [`briansmith/untrusted`](https://github.com/briansmith/untrusted) -- Safe, fast, zero-panic, zero-crashing, zero-allocation parsing of untrusted inputs in Rust

* [`maciejhirsz/logos`](https://github.com/maciejhirsz/logos) -- create ridiculously fast lexers

### Currying <a name = "curry"></a>

**Currying** is a way to produce higher order functions that contain some context that in turn can be applied to all passed in parameters.
* [Currying in Rust: Part 1](https://hashnode.com/post/currying-in-rust-cjpfb0i2z00cm56s2aideuo4z)
* [Currying in Rust: Part 2](https://hashnode.com/post/currying-in-rust-part-2-a-glimpse-of-generics-cjphbgun90025pms241ggh3d9)
* [Currying in Rust: Part 3](https://hashnode.com/post/currying-in-rust-part-3-the-circle-of-life-aka-why-borrowchecker-why-cjq3z1dd800dknds1sls4dqav)
# Testing

> [Thinking Above The Code](https://www.youtube.com/watch?v=-4Yp3j_jk8Q) by Leslie Lamport

**Tyler Neely (aka spacejam)**
* [RustFest Paris 2018: Building Reliable Infrastructure in Rust](https://www.youtube.com/watch?v=hMJEPWcSD8w)
* [Reliable Systems: Model-Based Property Testing](https://medium.com/@tylerneely/reliable-systems-series-model-based-property-testing-e89a433b360)

## Crates

* [`proptest`](https://crates.io/crates/proptest): it generates test according to properties we desire; nondeterminism in testing random values and deterministic replay for easy debugging (useful for compression and serialization; can also provide your own `proptest` macro generators)
* [`quickcheck`](https://crates.io/crates/quickcheck)
* [`model`](https://crates.io/crates/model)
* [mockiato](https://github.com/myelin-ai/mockiato)

*OS level*
* [`ptrace` command](http://man7.org/linux/man-pages/man2/ptrace.2.html)

## Building Reliable Infrastructure

* metrics and introspection
* capacity planning
* Brendan Gregg: Systems Performance: Enterprise and Cloud (chapter 2 specifically)
* Google: Site Reliability Engineering
* **Designing Data-Intensive Applications** by Martin Kleppman (*for* theory behind databases and distributed systems)

* *Rationality: From AI to Zombies*

* [Hermitage: Testing the I in ACID](http://martin.kleppmann.com/2014/11/25/hermitage-testing-the-i-in-acid.html) -- measuring and testing isolation levels...

## Model-Based Testing
> [Reliable System Series: Model-Based Testing](https://medium.com/@tylerneely/reliable-systems-series-model-based-property-testing-e89a433b360)

**Papers (very accessible)**
* Experiences with Quickcheck: Testing the Hard Stuff and Staying Sane
* Finding Race Conditions in Erlang with Quickcheck and PULSE

**TLA+**
* [Video Course on TLA+](https://lamport.azurewebsites.net/video/videos.html)
* [Learn TLA+ Guide](https://learntla.com/introduction/)
* [Lamport Homepage, TLA+](https://lamport.azurewebsites.net/tla/tla.html)
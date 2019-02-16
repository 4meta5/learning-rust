# Databases

## Cache

* [Append-only, on-disk key-value index](https://github.com/krl/appendix)
    * [writing an append-only, on-disk key-value store with lockless threads in Rust](https://github.com/krl/appendix/blob/master/description/writing.md)
* [Aster: cache proxy](https://github.com/wayslog/aster)
* [purpleposeidon/v11](https://github.com/purpleposeidon/v11) -- minimize wasted CPU cache by operating exactly on the data needed to solve a particular problem

### Heap Allocation

* [`enum_dispatch`](https://gitlab.com/antonok/enum_dispatch) -- near drop-in replacement for dynamic-dispatched method calls with up to 10x the speed
* [Storing unboxed trait objects in Rust](https://guiand.xyz/blog-posts/unboxed-trait-objects.html)
* [`Placement New`](http://blakesmith.me/2018/12/31/what-is-placement-new-in-rust.html)

**Garbage Collection**
* [Shifgrethor I: Garbage Collection as a Rust library](https://boats.gitlab.io/blog/post/shifgrethor-i/)
* [example -- kyren/luster](https://github.com/kyren/luster/blob/3e3a6ea12f6f523c105abd7fbe9d0ad226be784c/src/sequence.rs#L5-L36)


## Databases (and Wrappers)
* [`rust-bitcoin/hammserbald`](https://github.com/rust-bitcoin/hammersbald) -- fast, embedded blockchain database
* [Rust wrapper for Rocksdb](https://github.com/rust-rocksdb/rust-rocksdb)
* [`paritytech/paritydb`](https://github.com/paritytech/paritydb)
* [`tokv/tikv`](https://github.com/tikv/tikv) -- distributed transactional key-value database
* [Rust GraphQL Server Framework](https://github.com/nrc/graphql)
* [`graphprotocol/graph-node`](https://github.com/graphprotocol/graph-node) -- Graph Node indexes data from immutable datasources and serves it over GraphQL
* [How to Build a Modern Distributed Compute Platform](https://andygrove.io/how_to_build_a_modern_distributed_compute_platform/)
    * [DataFusion: Modern Distributed Compute Platform in Rust](https://github.com/andygrove/datafusion)
* [Rust at speed — building a fast concurrent database](https://www.reddit.com/r/rust/comments/acucrs/rust_at_speed_building_a_fast_concurrent_database/?st=JQJQZ8FS&sh=03abd45a) (Jon Hoo)
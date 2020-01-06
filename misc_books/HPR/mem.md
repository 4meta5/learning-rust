# Memory Management in Rust

Rust's borrow checker has three simple rules:
1. each binding (`name => value`) will have an owner
2. there can only one owner for a binding
3. when the owner goes out of scope, the binding is dropped


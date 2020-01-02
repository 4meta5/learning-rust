# Pascal's Talk and Blog Post

* [Pascal's talk at Rustfest Kiev April 2017]()
* [deterministic.space]

## Doc Tests

Well-documented == well-tested

Start lines with `#` to integrate imports without including them in the docs themselves.

## More Compiler Errors

* `#![deny(warnings, missing_docs)]`
* use CLIPPY

## Implementing the Right Traits

```rust
let x: IpAddress = [127, 0, 0, 1].into();
```

Makes it very easy to construct your types from standard types.

`std::convert` is your friend
* `AsRef` reference to reference conversions
* `From` and `Into` for value conversions
* `TryFrom`/`TryInto` for fallible conversions 

Implement
* `Debug`
* `(Partial)Ord`
* `(Partial)Eq`
*  `Hash`

* `Display`
* `Error`

* `Default`

* `FromStr` gives your the `parse()` function for data types

If you have a type that contains multiple values, implement `Iterator`.

## Session Types

* define a type for each state
* go from one state to another by returning a different type

## Extension Traits
# Zero Cost Abstractions

## Using Iterators

Using iterators to find the last element:
```rust
for arr in array_of_arrays {
    if let Some(elt) = arr.iter().rev().next() {
        println!("{}", elt);
    }
}
```

If we have to get an element in a specific position, then we should use the `get()` method. This maintains a double bound check.

```rust
for bar in barray_of_arrays {
    if let Some(foo) = bar.iter().get(173) {
        println!("{}", foo);
    }
}
```

However, this `.get()` call has a double bound check. It will first check if the index is correct to return a `Some(foo)` or `None`, and then the final check will verify that the returned element is `Some` or `None`.

If we have verified bound checking independently for the call, we can use `.getunchecked()` to get the element. Although this is unsafe to use, it is exactly equivalent to the C/C++ indexing operation, thereby allowing for higher performance when we know the element's location. Indeed, if we don't verify what we feed to get `unchecked`, an attacker could hypothetically access whatever is stored in the location even if it was a memory address outside the slice.

```rust
for bar in barray_of_arrays {
    // verify independently that 173 is before the end of the array
    println!("{}", unsafe { bar.iter().getunchecked(173)});
}
```

### Iterator Adaptors

The basic array type does not implement the `Iterator` trait, but a reference to the array is a slice, and slices implement the `IntoIterator` trait (which makes it usable as an iterator).

In the case of `u8`, it is actually better to copy them than to reference them.

The `collect()` method requires a type hint -- it can return any kind of collection, or specifically any type that implements the `FromIterator` trait.

* The `skip(n)` adaptor calls `next()` `n` times and discards what it returns; `take(n)` calls `take()` `n` times.
* `skip_while()` and `take_while()` will skip or take elements while the closure they run returns `true`.
* `map` and `filter`
* `fold`
* `sum`, `product`

* `cycle` makes the iterator start again once it gets to the end of the iterator

Let's say that we have two slices of the same length and want to generate a new vector with that same length, but with each element being the sum of the elements with the same index in the slice:

```rust
let arr1 = [10, 20, 30, 40, 50 ,60 100]
let arr2 = [100, 60, 50, 40, 30, 20, 10]
```

To generate a new array with each element being the sum of the elements in the same index for the input arrays.

```rust
let collection: Vec<_> = arr1.iter()
    .zip(arr2.iter())
    .map(|(foo1, foo2)| foo1 + foo2)
    .collect();
println!("{}", collection);
```

**External Crate: Itertools**

## Borrowing Degradations

You can do 3 things with variables when passing them to a function: send a reference (borrow), give the new function control of the variable (own), or copy/clone the variable to send it to a function.
* if you no longer require ownership of the variable, transfer it to the function
* if you still require it, send a reference
* if you require it and the API only accepts ownership, clone it

* *If it's <= size(`usize`) => copy*
* *if `usize` < `_size_` < 10 * `usize` => probably copy*
* bigger than that => reference

The greater the cyclomatic complexity of the code, the more difficult it is for the compiler to optimize the logic. With this in mind, it is *recommended* to create functions with no greater than 20-25 branches each (each branch represents a conditional like an `if` or `match` or `?`).

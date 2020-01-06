# Making Generics More Generic

**Functions as values** is the cornerstone of functional programming.

## Combinators Review

To chain two iterators, invoke `chain()`

```rust
(0..19).chain(10..20);
```

The `zip` function combines two iterators into tuple pairs, iterating until the end of the shortest iterator

```rust
(0..10).zip(10..20)
```

The `enumerate` function is a special case of `zip` that creates numbered tuples (0, a1),(1, a2), ...

```rust
(0..10).enumerate();
```

The `inspect` function applies a function to all values in the iterator during iteration

```rust
(0..10).inspect(|x| {
    println!("value {}", *x)
});
```

The `map` function applies a function to each element, returning the result in place

```rust
(0..10).map(|x| x*x);
```

The `filter` function restricts elements to those satisfying a predicate

```rust
(0..10).filter(|x| *x <3);
```

The `fold` function accumulates all values into a single result

```rust
(0..10).fold(0, |x,y| x+y);
```
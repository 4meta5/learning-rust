# Currying

**Currying** is a way to reduce *arity*, which is the amount of parameters passed to a function. 

Often times, the purpose of currying is to utilize the parameter to return a function (lambda) that contains the aforementioned parameter. *Currying is a way to produce higher order functions that contain some context that in turn can be applied to all input parameters.*

```rust
fn add<'a, T>(x: T) -> Box<Fn(T) -> T + 'a>
    where T: Add<Output=T> + Copy, T: 'a
{
    Box::new(move |y: T| x + y)
}
```

### References

* [Currying in Rust Part I](https://hashnode.com/post/currying-in-rust-cjpfb0i2z00cm56s2aideuo4z)
* [Currying in Rust, Part II](https://hashnode.com/post/currying-in-rust-part-2-a-glimpse-of-generics-cjphbgun90025pms241ggh3d9)
* [Currying in Rust, Part III](https://hashnode.com/post/currying-in-rust-part-3-the-circle-of-life-aka-why-borrowchecker-why-cjq3z1dd800dknds1sls4dqav)
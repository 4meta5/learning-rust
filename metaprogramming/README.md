# Metaprogramming

> basically just some notes on Rust Macros and best practices

## Declarative Macros

As an example, look at the simplified definition of the `vec!` macro:

```
#[macro_export]
macro_rules! vec {
    ( $( $x:expr),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}
```

The `*` following the comma specifies that the pattern matches zero or more of whatever precedes the `*`.

When we call this macro with ```vec![1, 2, 3]```, the `$x` pattern matches three times with the three expressions `1`, `2`, and `3`.

## Procedural Macros for Custom ```derive```

Procedural macros accept some Rust code as an input, operate on that code, and produce some Rust code as an output rather than matching against patterns and replacing the code with other code (as declarative macros do). 

At the moment, you can only define procedural macros to allow your traits to be implemented on a type by specifying the trait name in a ```derive``` annotation.

### References

* [The Rust Book - Appendix on Macros](https://doc.rust-lang.org/book/second-edition/appendix-04-macros.html)
* [The Little Book of Rust Macros](https://danielkeep.github.io/tlborm/book/mbe-README.html)
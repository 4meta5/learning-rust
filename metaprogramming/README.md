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

> The `+` in the argument list indicates that an argument may repeat at least once

When we call this macro with ```vec![1, 2, 3]```, the `$x` pattern matches three times with the three expressions `1`, `2`, and `3`.

**Designators**<br>
* `block`
* `expr` is used for expressions
* `ident` is used for variable/function names
* `item`
* `pat` (*pattern*)
* `path`
* `stmt` (*statement*)
* `tt` (*token tree*)
* `ty` (*type*)
* `vis` (*visibility qualifier*)

## Procedural Macros for Custom ```derive```

Procedural macros accept some Rust code as an input, operate on that code, and produce some Rust code as an output rather than matching against patterns and replacing the code with other code (as declarative macros do). 

At the moment, you can only define procedural macros to allow your traits to be implemented on a type by specifying the trait name in a ```derive``` annotation.

> macros are useful when we need to generate code at compile time that would otherwise require some analysis at runtime

To declare a crate as a procedural macro crate, you need functionality from the `syn` and `quote` crates, so we need to add them as dependencies; also need to set `proc-macro = true` under `[lib]` in the manifest file.

```
[lib]
proc-macro = true

[dependencies]
syn = "0.11.11"
quote = "0.3.15"
```


In the `src/lib.rs` file, we need something that looks like the following skeleton code:

```
extern crate proc_macro;
extern crate sync;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();

    // Parse the string representation
    let ast = syn::parse_derive_input(&s).unwrap();

    // Build the impl
    let gen = impl_hello_macro(&ast);

    // Return the generated impl
    gen.parse().unwrap()
}
```

Essentially we'll change what we put where the `impl_hello_macro` is called depending on the macro's purpose.

* the `proc_macro` crate comes with Rust (so we don't need to add it to the dependencies in the manifest file); it allows us to convert Rust code into a string containing that Rust code
* the `syn` crate parses Rust code from a string into a data structure that we can perform operations on
* the `quote` crate takes `syn` data structures and turns them back into Rust code

With this in mind, the `hello_macro_derive` function is called whenever a user of our library specifies `#[derive(HelloMacro)]` on a type. 

### References

* [The Rust Book - Appendix on Macros](https://doc.rust-lang.org/book/second-edition/appendix-04-macros.html)
* [Rust by Example: Macros](https://doc.rust-lang.org/rust-by-example/macros.html)
* [The Little Book of Rust Macros](https://danielkeep.github.io/tlborm/book/mbe-README.html)
# Metaprogramming
> [The Little Book of Rust Macros](https://danielkeep.github.io/tlborm/book/mbe-README.html)

Stages of compilation:
1. **tokenisation**: source text is transformed into a sequence of tokens (ie indivisible units)
2. **parsing**: the stream of tokens is turned into an Abstract Syntax Tree (AST)
* the AST contains the structure of the *entire* program, though it is based on purely *lexical* information. 
3. Macros are processed

> **Token trees** are between tokens and the AST!

* The input to every macro is a single non-leaf token tree
* Macros (actually, syntax extensions in general) are parsed as *part* of the abstract syntax tree

Macros can appear in place of the following:
* Patterns
* Statements
* Expressions
* Items
* `impl` Items

ie Macros can NOT appear in place of:
* Identifiers
* Match arms
* Struct fields
* Types (available in unstable Rust via `#![feature(typo)];`)

> *where* you can invoke a macro determines what its result will be interpreted as...the compiler takes the AST node and completely replaces the macro's invocation node with the output node. This is a *structural operation*, not a textural one!

Macro expansions are treated as AST nodes =>
* In addition to there being a limited number of invocation *positions*, macros can *only* exapnd to the kind of AST node the parser *expects* at that position
* As a consequence, macros *absolutely cannot* expand to incomplete or syntactically invalid constructs

> notes on [macro_rules](./macro_rules.md) from this same book...very useful!

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

* [Creating Macros in Rust](https://hub.packtpub.com/creating-macros-in-rust-tutorial/)

* [nymic](https://github.com/myrrlyn/nymic) -- crate for producing the type names of values

* [Procedural Macros in Rust 2018](https://blog.rust-lang.org/2018/12/21/Procedural-Macros-in-Rust-2018.html)

* [Deriving Traits in Rust with Procedural Macros](https://naftuli.wtf/2019/01/02/rust-derive-macros/)

* [`adhesion-rs`](https://github.com/ErichDonGubler/adhesion-rs) -- D-inspired contract programming in Rust using macros
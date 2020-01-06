# Coding Rust Cheatsheet (3 of 3)

Rust is a statically typed language, which means that it must know the types of all variables at compile time. The compiler can usually infer what type we want to use based on the value and how we use it.

These are notes from [The Rust Book](https://doc.rust-lang.org/book/), but they also may draw from [Steve Donovan's Gentle Intro](https://stevedonovan.github.io/rust-gentle-intro/readme.html). **These notes only cover the last 7 chapters of The Book** 

* [OOP](#oop)
* [Procedural Macros](#proceduralmacros)

## Object Oriented Programming Features of Rust <a name="oop"></a>

The *Gang of Four* book defines OOP as:
> Object-oriented programs are made up of objects. An *object* packages both data and the procedures that operate on that data. The procedures are typically called *methods* or *operations*.

Using this definition, Rust is object oriented: structs and enums have data, and ```impl``` blocks provide methods on structs and enums.

## Procedural Macros <a name="proceduralmacros">

At the moment, procedural macros need to be in their own crate. For a crate named ```foo```, a custom derive procedural macros is called ```foo-derive```. 

> As an example, if we wanted to define a procedural macro within ```hello-world```, we create a new crate called ```hello-world-derive``` inside the ```hello-world``` project.
```
$ cargo new hello-world-derive
```

To make sure that our ```hello-world``` crate is able to find this new crate we've created, we add it to our toml:
```
[dependencies]
hello-world-derive = { path = "hello-world-derive" } 
```

Here's the source of our ```hello-world-derive``` crate, here's an example:
```
extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_derive(HelloWorld)]
pub fn hello_world(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();

    // Parse the string representation
    let ast = syn::parse_derive_input(&s).unwrap();

    // Build the impl
    let gen = impl_hello_world(&ast);

    // Return the generated impl
    gen.parse().unwrap()
}
```
The first thing that we do is convert ```input: TokenStream``` to a ```String```. At the moment, the only thing you can do with a ```TokenStream``` is convert it to a string. 

But we need to be able to *parse* Rust code into something usable. We use ```syn``` as a crate for parsing Rust code. The other crate, ```quote```,  is the dual of ```syn``` -- it makes generating Rust code really easy. 

> We are going to take a ```String``` of the Rust code for the type we are deriving, parse it using ```syn```, construct the implementation of ```hello_world``` (using ```quote```), then pass it back to Rust compiler.

Now we can write ```impl_hello_world(&ast)```:
```
fn impl_hello_world(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    quote! {
        impl HelloWorld for #name {
            fn hello_world() {
                println!("Hello, World! My name is {}", stringify!(#name));
            }
        }
    }
}
```

The ```ast``` argument is a struct that gives us a representation of our type (which can be either a ```struct``` or an ```enum```). We are able to get the name of the type using ```ast.ident```.  The ```quote!``` macro lets us write up the Rust code that we wish to return and convert it into ```Tokens```. ```quote!``` lets us use some really cool templating mechanics; we write ```#name``` and ```quote!``` will replace it with the variable named ```name```. 

First, we need to add dependencies for ```syn``` and ```quote``` in the ```Cargo.toml``` for ```hello-world-derive```.

```
[dependencies]
syn = "0.11.11"
quote = "0.3.15"
```

To declare that our ```hello-world-derive``` crate is a ```proc-macro``` crate type, we use this syntax within the ```Cargo.toml``` file:
```
[lib]
proc-macro = true
```

> there's some more stuff in the [here](https://joshleeb.com/posts/rust-procedural-macros/) and [here](https://doc.rust-lang.org/nightly/reference/procedural-macros.html)
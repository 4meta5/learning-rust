# Big Arithemtic

Working precisely with big or small numbers is not easy in a language like Rust. The expressive type system helps here, but this is by no means a solved problem.

## code

* [`apint`](https://github.com/Robbepop/apint)
* [`num-bigint`](https://github.com/rust-num/num-bigint)

## blog posts

Fortunately, Rust's type system is quite expressive so we can define behavior in a very explicit way, especially when it comes to type-specific operations and conversions:
* [Math with distances in Rust: safety and correctness across units](https://ferrisellis.com/content/rust-implementing-units-for-types/#closing-thoughts)
* [Convenient and Idiomatic Conversions in Rust](https://ricardomartins.cc/2016/08/03/convenient_and_idiomatic_conversions_in_rust)
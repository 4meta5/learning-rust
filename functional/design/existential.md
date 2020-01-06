# Existential Types in Rust
*Notes on [Existential types in Rust](https://varkor.github.io/blog/2018/07/03/existential-types-in-rust.html), [new perspective on impl Trait](https://varkor.github.io/blog/2018/07/04/a-new-perspective-on-impl-trait.html) by `varkor`*

An **existential type** is a type that represents *any type satisfying a given property* `=>` any type implementing a given trait.

Any time we see the syntax specifying as output of some function `impl Trait`, substitute `∃ T. T: Trait`, such that there exists some type `T` that implements the trait `Trait`. So `fn(A, B, C) -> impl Trait` is equivalent to `fn(A, B, C) -> (∃ T. T: Bar)`.

Rust has long has **universally-quantified types** in the form of generic parameters. `fn foo<S, T>(s: S, t: T) -> T` is the syntax for a type `∀ S, T. (fn(S, T) -> T)`.

## Argument Position `impl Trait`

```rust
fn foo() -> impl Foo; // Return position `impl Trait` (RPIT)
fn bar(impl Bar) -> (); // Argument position `impl Trait` (APIT)
```

If the return-position `impl Foo` is an existential type, what does that make the argument-position `impl Bar`?

`fn(impl Foo) -> T` is actually equivalent (*isomorphic*) to `fn<S: Foo>(S) -> T` and we can freely convert between them.

**Proof**<br>
Well, let's consider the following proposition in intuitionist logic: `((∃ x. P(x)) -> Q) <-> (∀ x. (P(x) -> Q))`. If we consider the proposition as a type, it also holds, by the **Curry-Howard Isomorphism** (sometimes referred to by CS people as the *Currey-Howard Equivalence*).

Therefore, `impl Trait` is always existential. It is not universal in the argument position, but it is just conveniently isomorphic to a very similar universally quantified type.

## Restriction on Return Position `impl Trait`

```rust
trait Trait {}
struct A;
struct B;
impl Trait for A {}
impl Trait for B {}

fn wont_compile(foo: bool) -> impl Trait {
    if foo { A } else { B } // ERROR incompatible types
}
```

The Rust compiler needs to know which (unquantified) type will be returned. The existential type doesn't exist at runtime -- it must pick a specific unquantified type.

This changes the semantics of our existentials. For each instance of `impl Trait` in the return position, we can only allow a single unquantified type to represent it. 

**APIT is tightly-bound whereas RPIT is bound at the level of the whole function.**

## Enter `dyn Trait`

`dyn Trait` provides existential quantification (although I think it still has to be boxed, ie placed behind some pointer).

The difference is the representation (in the compiler): because `dyn Trait` is truly existential, it must hold information about the specific type `T` at runtime (which is a disadvantage that `impl Trait` doesn't have).


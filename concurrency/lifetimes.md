# Lifetimes

**Types to Focus On Learning**
* `Weak<T>` vs `Strong<T>`
* `Rc<T>` vs `Arc<T>`
* `Cow<T>` vs `Box<T>`
* `Mutex<T>` vs. `RwLock<T>` vs. `atomic<T>`
* `Send` vs [`Sync`](https://doc.rust-lang.org/std/marker/trait.Sync.html) traits


*Misc Notes*
* a type conforms to `Sync` => it is safe to share references to this type between threads
* `std::cell::Cell<T>` and `RefCell<T>`

## Lifetimes Notes

Lifetimes are all about managing the scope of references. When we see a `struct`, `closure`, or `enum` with lifetime parameters, the lifetimes refer to the fields of the given data structure, not the structure itself.

* multiple lifetimes can satisfy some given lifetime parameter and the compiler will take the minimal one
* simple Rust types do not have *subtyping* unless they have lifetime parameters

### Subtyping
> [Subtyping and coercion](https://featherweightmusings.blogspot.com/2014/03/subtyping-and-coercion-in-rust.html)

Subtyping is a relation on types such that a type `T` is a subtype of `U` if `T` is, in some sense, more specific than `U`. More formally, if `T` and `U` each denote a set of values, the set of values denoted by `T` is a subset of the set of values denoted by `U`.

**Inclusive Polymorphism** (Liskov substitution principle `||` *strong behavioral typing*): if an expression is of type `T` such that `T` is a subtype of `U`, then type `T` can be used anywhere that requires a parameter of type `U`.

**Coercion** is an operation on values (or expressions) where a value of type `T` can be changed in some way to a value of type `U` => coercion requires the insertion of code which does the low-level conversion from `T` to `U` while subtyping is really just a compiler check.

The key difference between subtyping and coercion is that coercion changes the underlying type while subtyping does not.

**Lifetimes** are just regions of code, and regions of code can be partially ordered with the *contains* (outlives) relationship. For lifetimes, the bigger region is a subtype of the smaller region (because the subtype has at minimum the lifetime of its supertype). The `'static` lifetime is a subtype of every lifetime (because it outlives everything).

### Variance

Variance specifies a set of rules governing how subtyping should compose. 

A *type constructor* is any generic type with unbounded arguments. 
* `Vec` is a type constructor that takes a type `T` and returns a type `Vec<T>`
* `&` and `&mut` are type constructors that take two inputs: a lifetime and a type to point to

Similar to the [src](https://doc.rust-lang.org/stable/nomicon/subtyping.html) of these notes, we'll refer to `F<T>` as a type constructor to reason about the behavior of `T`.

A type constructor `F`'s variance is how the subtyping of its inputs influence the subtyping of its outputs. Given `Sub`, `Super` such that `Sub` is a subtype of `Super`, 
* `F` is covariant if `F<Sub>` is a subtype of `F<Super>`
* `F` is contravariant if `F<Sub>` is a supertype of `F<Super>`
* `F` is invariant if there is no subtyping relationship between `F<Sub>` and `F<Super>`

When `F` has multiple type parameters, we can reason about each independently such that `F<T, U>` might be covariant over `T` and invariant over `U`.

> Invoking contravariance involves higher-order programming with function pointers that take references with specific lifetimes (as opposed to the usual "any lifetime", which gets into higher rank lifetimes, which work independently of subtyping). - [understand this from src later](https://doc.rust-lang.org/stable/nomicon/subtyping.html) 

## Lifetime Elision Rules

Lifetimes on function or method parameters are called input lifetimes, and lifetimes on return values are called output lifetimes.

1. each parameter that is a reference gets its own lifetime parameter
2. if there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters
3. if there are multiple input lifetime parameters, but one of them is `&self` or `&mut self` because this is a method, the lifetime of self is assigned to all output lifetime parameters

## References

* [The Book](https://doc.rust-lang.org/book/ch19-02-advanced-lifetimes.html)
* [The Rustonomicon](https://doc.rust-lang.org/stable/nomicon/)
* [Understanding Rust Lifetimes](https://medium.com/nearprotocol/understanding-rust-lifetimes-e813bcd405fa)
* [You Can't Rust That](http://lucumr.pocoo.org/2018/3/31/you-cant-rust-that/)
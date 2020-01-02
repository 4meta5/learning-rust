# Trait Notes

## Method Collision with Traits

```rust
<Object as TraitUsed>::method_name();
```

## Associated Constants

```rust
// appending a constant log before every result print 
trait SuperLog {
    const LABEL: Display;

    fn log(&self, f: &mut fmt::Formatter) -> fmt::Result;
}
```

## Associated Types
*an additional constraint*

## Call Me Maybe

```rust
struct Abstraction<T>(T);

impl<T> Abstraction<T> {
    fn map<U, F> (self, f: F) -> Abstraction<U>
        where F: FnOnce(T) -> U
    {
        Abstraction(f(self.0))
    }
}
```

* `FnOnce` is the most generic trait bound (everything can be called at least once)
* to call the function more than once, use `FnMut`
* `Fn` is fairly rare as it provides few additional options to the calling function
* when returning a closure, `Fn` is the most general since it allows calling the object multiple times and behind any kind of reference

## Examples

```rust
trait FnOnce {
    type Output;
    fn call_once(self) -> Self::Output;
}

type FnMut: FnOnce {
    fn call_mut(&mut self) -> Self::Output;
}

type Fn: FnMut {
    fn call(&self) -> Self::Output;
}
```

## References

* [A Generalist's View of Traits](https://www.youtube.com/watch?v=3YCqgwpuFM0)
* [Fn Trait](https://www.youtube.com/watch?v=9PIn4suU3jM)
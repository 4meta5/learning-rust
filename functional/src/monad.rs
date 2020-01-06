/// higher-kinded types
/// https://varkor.github.io/blog/2019/03/28/idiomatic-monads-in-rust.html

trait Monad<A>: Functor<A> {
    trait SelfTrait<T>;

    // Unit
    type Unit<T>: Monad<T> + SelfTrait<T>;

    fn unit(A) -> Unit<A>;

    // Bind
    type Bind<T, F>: Monad<T> + SelfTrait<T>;

    trait BindFn<T, U>;

    fn bind<B, MB: Self::SelfTrait<B>, F: Self::BindFn<A, MB>>(Self, F) -> Self::Bind<B, F>;
}


impl<A> Monad<A> for Option<A> {
    trait SelfTrait<T> = Id<Option<T>>;

    // Unit
    type Unit<T> = Option<T>;

    fn unit(a: A) -> Option<A> {
        Some(a)
    }

    // Bind
    type Bind<T, F> = Option<T>;

    trait BindFn<T, U> = FnOnce(T) -> U;

    fn bind<B, MB: Id<Option<B>>, F: FnOnce(A) -> MB>(self, f: F) -> Option<B> {
        self.and_then(f)
    }
}

// Implementing `Monad` for a trait.
impl<A, I: Iterator<Item = A>> Monad<A> for I {
    trait SelfTrait<T> = Iterator<Item = T>;

    // Unit
    type Unit<T> = iter::Once<T>;

    fn unit(a: A) -> iter::Once<A> {
        iter::once(a)
    }

    // Bind
    type Bind<T, F> = iter::FlatMap<T, F>;

    trait BindFn<T, U> = FnMut(T) -> U;

    fn bind<B, MB: Iterator<Item = B>, F: FnMut(A) -> B>(self, f: F) -> iter::FlatMap<B, F> {
        self.flat_map(f)
    }
}

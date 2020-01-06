/// Applicative

trait Applicative<A>: Functor<A> {
    trait SelfTrait<T>(T):

    // Unit
    fn unit(A) -> Self;

    type Apply<T, F>: Applicative<T>;

    trait BindFn<T, U>;

    fn apply<B, F: BindFn<A, B>, T: SelfTrait<F>>(T, Self) -> Apply<B, T>;
}
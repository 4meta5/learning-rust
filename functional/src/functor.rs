/// https://varkor.github.io/blog/2018/08/28/feasible-functors-in-rust.html
/// 
trait Functor<G: Func> {
    // This is pseudo-syntax for an associated trait.
    trait MapOb<A>;
    // I'm uncurrying `map_mor` here to avoid requiring that we add currying
    // to Rust to support this pattern.
    // Notice how the particular function variant we're using is abstracted
    // into a trait parameter on `Functor`.
    fn map_mor<A, B>(
        xa: impl MapOb<A>,
        f: impl G(A) -> B,
    ) -> impl MapOb<B>;
}

// Now we can define an implementation of a trait *for* a trait.
// This might be a slightly confusing concept at first, but the
// definitions themselves are very straightforward: we're essentially
// forwarding everything to `Iterator`, which already has all the
// information we need.
// `Iterator`'s map takes a `FnMut`, so we pass it in explicitly here.
impl Functor<FnMut> for Iterator {
    trait MapOb<A> = Iterator<Item = A>;

    fn map_mor<A, B>(
        xa: impl MapOb<A>,
        f: impl FnMut(A) -> B,
    ) -> impl MapOb<B> {
        <MapOb<A> as Iterator<Item = A>>::map(xa, f)
    }
}
// extending https://shingtaklam1324.github.io/number-theory-rust/
#[derive(Debug,Clone, Copy)]
struct Z;
#[derive(Debug, Clone, Copy)]
struct S<T>(T); // wrapper for numbers (to make successor function work)

#[derive(Debug, Clone, Copy)]
struct Nat<T>(T); // natural numbers presumably

const ZERO: Nat<Z> = Nat(Z); // the value zero `=>` S(Z) is one in this example

fn successor<A>(d: Nat<A>) -> Nat<S<A>> {
    Nat(S(d.0))
} // counting would 1, 2, 3, 4... ie Nat(Z), Nat(S(Z)), Nat(S(S(Z))), Nat(S(S(S(Z))))...

// for real-time testing purposes
fn main() {
    println!("{:?}", successor(ZERO));
    println!("{:?}", successor(successor(ZERO)));
}

// this testing will not work because the equality methods are particularly unwieldy 
// -- for this reason, give it a go...
#[cfg(test)]
mod tests {
    use super::*;
    fn successor_works() {
        assert_eq!(successor(Nat(Z)), successor(ZERO));
    }
}
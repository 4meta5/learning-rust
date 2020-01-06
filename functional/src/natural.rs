// Natural Numbers
use std::marker::PhantomData;

struct Zero;
struct Succ<N: Nat>(PhantomData<N>);

trait Nat {}

impl Nat for Zero {}
impl<N: Nat> Nat for Succ<N> {}

// dependent vector type (dependent on natural numbers)
struct Vector<N: Nat, A>(Vec<A>, PhantomData<N>);

fn main() {
    let _zero: Zero;
    let _one: Succ<Zero>;

    // vector that is dependent on the natural numbers implementation
    let v: Vector<Zero, u8> = Vector::<Zero, u8>::new();
    let v_prime: Vec<Succ<Zero>, u8> = v.cons(1);
}
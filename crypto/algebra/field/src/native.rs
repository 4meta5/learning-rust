//! Trivial native modular field

use super::field::*; // get the Field trait
// if I change the Field trait in field.rs
// I need to change the `impl` later as well

#[derive(Copy, Clone, Debug)]
pub struct Value(i64);

/// Trivial implementation of a field using i64 values and performing
/// native modulo reduction after each operation
///
/// actual values shouldn't exceed the u32 or i32 ranges as multiplication
/// is naive
///
/// this is really just a reference implementation
pub struct NativeField(i64);

impl Field for NativeField {
    type U = Value;

    fn new(prime: u64) -> NativeField {
        NativeField(prime as i64)
    }

    fn modulus(&self) -> u64 {
        self.0 as u64
    }

    fn from_u64(&self, a: u64) -> Self::U {
        Value(a as i64 % self.0)
    }

    fn to_u64(&self, a: Self::U) -> u64 {
        a.0 as u64
    }

    fn add(&self, a: Self::U, b: Self::U) -> Self::U {
        Value((a.0 + b.0) % self.0)
    }

    fn sub(&self, a: Self::U, b: Self::U) -> Self::U {
        let tmp = a.0 - b.0;
        if tmp > 0 {
            Value(tmp)
        } else {
            Value(tmp + self.0)
        }
    }

    fn mul(&self, a: Self::U, b: Self::U) -> Self::U {
        Value((a.0 * b.0) % self.0)
    }

    fn inv(&self, a: Self::U) -> Self::U {
        let tmp = ::numtheory::mod_inverse((a.0 % self.0) as i64, self.0 as i64);
        self.from_i64(tmp)
    }
}
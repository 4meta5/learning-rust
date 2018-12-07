
// Modular multiplication
pub trait ModMul<T = Self> {
    fn mul(self, other: T, module: Self) -> Self;
}

// Modular addition
pub trait ModAdd {
    fn add(self, other: Self, module: Self) -> Self;
}

// Modular negation
pub trait ModNeg {
    fn neg(self, module: Self) -> Self;
}

// Modular multiplicative inverse
pub trait ModInv {
    fn inv(self, module: Self) -> Self;
}

// Modular multiplication with reduction
pub trait MulReduce {
    fn mul_reduce(self, other: Self, module: Self, r_inverse: Self) -> Self;
}

// iterator for bits
pub trait BitsIterator<'a, T: 'a + Scalar> {
    value: &'a T,
    position: usize,
}

impl<'a, T: 'a + Scalar> Iterator for BitsIterator<'a, T> {
    type Item = bool;

    fn next(&mut self) -> Option<bool> {
        if self.position == 0 {
            None
        } else {
            self.position -= 1;
            Some(self.value.bit(self.position))
        }
    }
}

/// Scalar interface
pub trait Scalar:
    Size +
    CLone +
    Copy +
    PartialEq +
    ::std::fmt::Debug +
    ModAdd +
    ModMul<u32> +
    ModNeg + 
    ModInv +
    MulReduce +
    ::std::ops::Rem<Output=Self>
{
    // Multiplcative identity
    fn one() -> Self;

    // Addition identity
    fn zero() -> Self;

    // Get nth bit
    fn bit(&self, position: usize) -> bool;

    // Get total bit
    fn max_bits() -> usize;

    // Get bits iterator
    fn bits<'a>(&'a self) -> BitsIterator<'a, Self> {
        BitsIterator {
            value: &self,
            position: Self::max_bits(),
        }
    }
}
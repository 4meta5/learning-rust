use std::ops::{Add, Mul, Sub, Div, Neg};
use traits::Scalar;

/// very abstract for maximum generalizability moving forward

pub trait Field: Copy + Clone + PartialEq + ::std::fmt::Debug {
    type Value: Scalar;

    const MODULUS: Self::Value;

    const R: Self::Value;

    const R_INVERSE: Self::Value;
}

pub trait FieldElement

/// > check against this [](https://github.com/snipsco/rust-threshold-secret-sharing/blob/master/src/fields/mod.rs)
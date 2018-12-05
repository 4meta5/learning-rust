use std::ops::{Add, Mul, Sub, Div, Neg};
use arith::Scalar;

/// very abstract for maximum generalizability moving forward

pub trait Field: Copy + Clone + PartialEq + ::std::fmt::Debug {
    type Value: Scalar;

    const MODULUS: Self::Value;

    const R: Self::Value;

    const R_INVERSE: Self::Value;
}

pub trait FieldElement
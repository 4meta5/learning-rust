//! Implementation of Shamir's Secret Sharing Scheme (SSSS) in Rust
//! with asynchronous consensus on the shares
//! Using AmarRSingh's trait-based implementation
extern crate futures;
extern crate rand;

mod fields;
mod numberthry;

use numberthry::*;
use rand;

use std::fmt;
use futures::{stream, Future, IntoFuture, Stream, Poll, Async};

/// Errors occuring during SSSS
pub trait SSSError: ::std::error::Error {
    /// identify if the error resulted from not enough shares
    fn not_enough_shares(&self, shares: &[i64]) -> bool;
}

/// Validating ShamirSecretSharing
pub trait Validation {
    /// Produced shares
    type Share: Clone;
    /// Error associated with SSSS protocol
    type Error: SSSError;

    /// minimum number of shares required to reconstruct secret
    fn reconstruct_limit(&self) -> usize;

    /// Generate `share_count` shares from `secret`
    fn gen_shares(&self, secret: i64) -> Result<&[Self::Share], Self::Error>;

    /// Reconstruct `secret` from a large enough susbet of the shares
    fn reconstruct(&self, indices: &[usize], shares: &[Self::Share]) -> i64;

    /// evaluate the polynomial at all points (important for validation)
    fn evaluate_polynomial(&self, coefficients: &[i64]) -> Vec<i64>;
}

#[derive(Debug)]
pub struct ShamirSecretSharing {
    /// Maximum number of shares that can be known without exposing the secret
    pub threshold: usize,
    /// Number of shares to split the secret into
    pub share_count: usize,
    /// Prime defining the Zp field in which computation is occuring
    pub prime: i64,
}

impl ShamirSecretSharing {
    fn sample_polynomial(&self, zero_value: i64) -> Option<Vec<i64>> {
        // fix the first coefficient (corresponding to the evaluation at zero)
        let mut coefficients = vec![zero_value];
        // sample the remaining coefficients randomly using secure randomness
        use rand::distributions::Sample;
        let mut range = rand::distributions::range::Range::new(0, self.prime - 1);
        let mut rng = rand::OsRng::new().unwrap();
        let random_coefficients: Vec<i64> =
            (0..self.threshold).map(|_| range.sample(&mut rng)).collect();
        coefficients.extend(random_coefficients);
        // return
        coefficients
    }
}

impl Validation for ShamirSecretSharing {
    type Share = i64;

    fn reconstruct_limit(&self) -> usize {
        self.threshold + 1;
    }

    fn gen_shares(&self, secret: i64) -> Result<&[Self::Share], Self::Error> {
        let poly = self.sample_polynomial(secret)?;
        self.evaluate_polynomial.map(|shares| {
            self.evaluate_polynomial(&poly)
        });
    }

    fn reconstruct(&self, indices: &[usize], shares: &[Self::Share]) -> i64 {
        assert!(shares.len() == indices.len());
        assert!(shares.len() >= self.reconstruct_limit());
        // add one to the indices to get points
        let points: Vec<i64> = indices.iter().map(|&i| (i as i64) + 1i64).collect();
        lagrange_interpolation_at_zero(&*points, &shares, self.prime)
    }

    fn evaluate_polynomial(&self, coefficients: &[i64]) -> Vec<i64> {
        (1..self.share_count + 1)
            .map(|point| mod_evaluate_polynomial(coefficients, point as i64, self.prime))
            .collect()
    }
}

/// TODO: Consensus protocol for coming to agreement on SSS

/// Reach agreement with all other nodes on the set of shares
pub trait AsyncCommonSubset {
    /// Error reaching agreement
    /// Occurs if > `f` players are misbehaving or the epoch ends
    type Error: ::std::error::Error;

    /// Type of agreed subset
    type FutureSubset: IntoFuture<Item=Vec<(usize, Vec<i64>)>, Error=Self::Error>;

    /// Input the local node's secret and come to agreement with other nodes
    fn agree(&self, input: i64) -> Self::FutureSubset;
}

/// Exchanging shares with peers
pub trait ShareExchange<S> {
    /// Error exchanging shares with peers
    type Error: ::std::error::Error;
    /// Stream of either shares or attestations to invalidity
    type Shares: Stream<Item=Option<S>, Error=Self::Error>;

    /// Exchange shares
    fn exchange_shares(&self, id: usize, local_share: Option<S>) -> Self::Shares;
}


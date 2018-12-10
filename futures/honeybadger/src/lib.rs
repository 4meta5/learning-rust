//! Implementation of HoneyBadgerBFT in Rust
//! Using Rphmeier's trait-based implementation

extern crate futures;

use std::fmt;

use futures::{stream, Future, IntoFuture, Stream, Poll, Async};

// Errors occurring from threshold decryption
pub trait ThresholdDecryptionError: ::std::error::Error {
    fn invalid_shares(&self) -> Option<&[usize]>;
}

// A threshold encryption scheme
pub trait ThresholdEncryption {
    /// Produced Shares
    type Share: Clone;
    /// Error on creating a decryption share or combining them
    type Error: ThresholdDecryptionError;

    /// How many shares are required to decrypt.
    /// Should be equal to `f+1`.
    fn threshold(&self) -> usize;

    /// Encrypt a plaintext
    fn encrypt(&self, plaintext: &[u8]) -> Vec<u8>;

    /// Whether a ciphertext, share combination is good.
    fn share_good(&self, ciphertext: &[u8], share: &Self::Share) -> bool;

    /// Create a decryption share. Fails if ciphertext is malformed.
    fn decrypt_share(&self, ciphertext: &[u8]) -> Result<Self::Share, Self::Error>;

    /// Combine decryption shares. Fails if there are fewer than threshold valid shares
    /// or the ciphertext is invalid
    fn decrypt(&self, ciphertext: &[u8], shares: &[Self::Share]) -> Result<Vec<u8>, Self::Error>;
}

/// Reach agreement with all other nodes on the set of (potentially invalid) ciphertexts
pub trait AsyncCommonSubset {
    /// Error reaching agreement. This shouldn't occur unless the epoch ends or more than
    /// 'f' players are misbehaving
    type Error: ::std::error::Error;

    /// Type of agreed subset
    type FutureSubset: IntoFuture<Item=Vec<(usize, Vec<u8>)>, Error=Self::Error>;

    /// Input the local node's ciphertext and come to agreement with the other nodes
    fn agree(&self, Input: &[u8]) -> Self::FutureSubset;
}

/// Exchanging decryption shares with peers
pub trait ShareExchange<S> {
    /// Error exchanging decryption shares with peers.
    type Error: ::std::error::Error;
    /// Stream of eother shares of attestations to ciphertext invalidity
    type Shares: Stream<Item=Option<S>, Error=Self::Error>;

    fn exchange_shares(&self, id: usize, local_share: Option<S>) -> Self::Shares;
}

/// The protocol honey badger is being run for..
pub trait Protocol {
    /// Error decoding proposal
    type Error: ::std::Error::Error;
    /// The proposal, drawn from a buffer
    type Proposal: Into<Vec<u8>>;
    /// The block type
    type Block;

    /// Decode a proposal
    fn decode_proposal(data: &[u8]) -> Result<Self::Proposal, Self::Error>;

    /// Combine a set of proposals into a block in such a way that ordering does not matter
    fn combine_proposals<I: IntoIterator<Item=Self::Proposal>>(proposals: I) -> Self::Block;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct BadCipherText;

// more code...need to add later
/// I want to understand futures better first


//! Implementation of HoneyBadgerBFT in Rust
//! Using Rphmeier's trait-based implementation

extern crate futures;

use std::fmt;

use futures::{stream, Future, IntoFuture, Stream, Poll, Async};

/// Errors occurring from threshold decryption
pub trait ThresholdDecryptionError: ::std::error::Error {
    /// if this error resulted from a failed threshold decryption,
    /// identify the indices of the invalid shares
    fn invalid_shares(&self) -> Option<&[usize]>;
}

/// A threshold encryption scheme
pub trait ThresholdDecryption {
    /// Produced shares
    type Share: Clone;
    /// Error on creating a decryption share or combining them.
    type Error: ThresholdDecryptionError;

    /// How many shares are required to decrypt
    /// Should be equal to `f + 1`
    fn threshold(&self) -> usize;

    /// Encrypt a plaintext
    fn encrypt(&self, plaintext: &[u8]) -> Vec<u8>;

    /// Whether a ciphertext, share combination is good
    fn share_good(&self, ciphertext: &[u8], share: &Self::Share) -> bool;

    /// Create a decryption share. Fails if ciphertext is malformed
    fn decrypt_share(&self, ciphertext: &[u8]) -> Result<Self::Share, Self::Error>;

    /// Combine decryption shares. Fails if there are fewer than `threshold` valid shares
    /// or if the ciphertext is invalid
    fn decrypt(&self, ciphertext: &[u8], shares: &[Self::Share]) -> Result<Vec<u8>, Self::Error>;
}

/// Reach agreement with all other nodes on the set of (potentially invalid) ciphertexts
pub trait AsyncCommonSubset {
    /// Error reaching agreement
    /// only occurs if > `f` players are misbehaving or the epoch ends
    type Error: ::std::error::Error;

    /// Type of agreed subset
    type FutureSubset: IntoFuture<Item=Vec<(usize, Vec<u8>)>, Error=Self::Error>;

    /// Input the local node's ciphertext and come to agreement with the other nodes
    fn agree(&self, input: &[u8]) -> Self::FutureSubset;
}

/// Exchanging decryption shares with peers
/// ... if the ciphertext is invalid, we have to broadcast a message indicating
/// its invalidity
pub trait ShareExchange<S> {
    /// Error exchanging decryption shares with peers
    type Error: ::std::error::Error;
    /// Stream of either shares or attestations to ciphertext invalidity
    type Shares: Stream<Item=Option<S>, Error=Self::Error>;

    fn exchange_shares(&self, id: usize, local_share: Option<S>) -> Self::Shares;
}

/// The protocol honey badger is being run for
pub trait Protocol {
    /// Error decoding proposal
    type Error: ::std::error::Error;
    /// The proposal, drawn from a buffer
    type Proposal: Into<Vec<u8>>;
    /// The block type
    type Block;

    /// Decode a proposal
    fn decode_proposal(data: &[u8]) -> Result<Self::Proposal, Self::Error>;

    /// Combine a set of proposals into a block in such a way that ordering doesn't matter
    fn combine_proposals<I: IntoIterator<Item=Self::Proposal>>(proposals: I) -> Self::Block;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct BadCiphertext;

impl ::std::error::Error for BadCiphertext {
    fn description(&self) -> &str { "Bad ciphertext, according to 2t + 1" }
}

impl fmt::Display for BadCiphertext {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Bad ciphertext")
    }
}

struct Accumulating<T: ThresholdEncryption> {
    ciphertext: Vec<u8>,
    tpke: T,
    bad_votes: usize, 
    good_shares: Vec<T::Share>,
}

impl<T: ThresholdEncryption> Accumulating<T> {
    fn accumulate(mut self, share: Option<T::Share>) -> ShareAccumulatorState<T> {
        match share {
            None => {
                self.bad_votes += 1;

                let f = self.tpke.threshold() - 1;
                let needed_votes = 2 * f + 1;

                if self.bad_votes >= needed_votes {
                    return ShareAccumulatorState::Bad;
                }
            }
            Some(share) => {
                if self.tpke.share_good(&self.ciphertext, &share) {
                    self.good_shares.push(share);
                    if self.good_shares.len() >= self.tpke.threshold() {
                        return match self.tpke.decrypt(&self.ciphertext, &self.good_shares) {
                            Ok(plaintext) => ShareAccumulatorState::Good(plaintext),
                            Err(_) => ShareAccumulatorState::Bad, // shares were checked before, shouldn't happen
                        }
                    }
                }
            }
        }

        ShareAccumulatorState::Accumulating(self)
    }
}

enum ShareAccumulatorState<T: ThresholdEncryption> {
    Done,
    Accumulating(Accumulating<T>),
    Bad,
    Good(Vec<u8>),
}

struct ShareAccumulator<T: ThresholdEncryption, I> {
    state: ShareAccumulatorState<T>,
    inner: I,
}

impl<T: ThresholdEncryption, I> ShareAccumulator<T, I> {
    // create a share accumulator with threshold (assume to be t + 1)
    fn create(tpke: T, ciphertext: Vec<u8>, inner: I, local_vote: Option<T::Share>) -> Self {
        let threshold = tpke.threshold();
        let a = Accumulating {
            ciphertext,
            tpke,
            bad_votes: 0,
            good_shares: Vec::with_capacity(threshold),
        };

        ShareAccumulator {
            inner,
            state: a.accumulate(local_vote),
        }
    }
}

impl<T, I> Future for ShareAccumulator<T, I> where T: ThresholdEncryption, I: Stream<Item=Option<T::Share>> {
    type Item = Vec<u8>;
    type Error = BadCiphertext;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        loop {
            match ::std::mem::replace(&mut self.state, ShareAccumulatorState::Done) {
                ShareAccumulatorState::Done => panic!("poll after finish"),
                ShareAccumulatorState::Good(p) => return Ok(Async::Ready(p)),
                ShareAccumulatorState::Bad => return Err(BadCiphertext),
                ShareAccumulatorState::Accumulating(acc) => {
                    match self.inner.poll() {
                        Ok(Async::NotReady) => {
                            self.state = ShareAccumulatorState::Accumulating(acc);
                            return Ok(Async::NotReady)
                        }
                        Ok(Async::Ready(Some(val))) => {
                            self.state = acc.accumulate(val);
                        }
                        Ok(Async::Ready(None)) => {
                            return Err(BadCiphertext);
                        }
                        Err(_) => {
                            self.state = ShareAccumulatorState::Accumulating(acc);
                        }
                    }
                }
            }
        }
    }
}

/// Perform an epoch of HoneyBadgerBFT
pub fn honey_badger_bft<'a, P, T, A, S>(proposal: P::Proposal, tpke: T, acs: A, se: S) -> Result<P::Block, Box<::std::error::Error + 'a>>
    where
        P: Protocol,
        T: ThresholdEncryption + Clone,
        A: AsyncCommonSubset,
        S: ShareExchange<T::Share>,

        P::Error: 'a,       
        A::Error: 'a,
{
    let proposal_bytes = proposal.into();
    let encrypted_proposal = tpke.encrypt(&proposal_bytes);

    let future = acs
        .agree(&encrypted_proposal)
        .into_future()
        .map_err(|e| Box::new(e) as Box<_>)
        .and_then(move |agreed_ciphertexts| {
            let mut accumulating = Vec::new();
            for (origin, ciphertext) in agreed_ciphertexts {
                let decrypt_share = match tpke.decrypt_share(&ciphertext) {
                    Ok(share) => Some(share),
                    Err(_) => None,
                };

                let inner = se.exchange_shares(origin, decrypt_share.clone());

                accumulating.push(ShareAccumulator::create(
                    tpke.clone(), 
                    ciphertext, 
                    inner, 
                    decrypt_share
                ).map_err(|e| Box::new(e) as Box<_>));
            }

            // the `then` call right now swallows errors so fold doesn't exit
            // prematurely.
            //
            // further on, we can return a (Block, JustifiedError) from the future.
            stream::futures_unordered(accumulating)
                .and_then(|p| P::decode_proposal(&p).map_err(|e| Box::new(e) as Box<::std::error::Error>))
                .then(|item| Ok(item.ok())) 
                .fold(Vec::new(), |mut v, item| {
                    if let Some(item) = item { v.push(item) }
                    Ok::<_, Box<::std::error::Error>>(v)
                })
                .map(P::combine_proposals)
        });

    future.wait()
}
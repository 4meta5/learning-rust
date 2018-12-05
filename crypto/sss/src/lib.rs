#[macro_use]
extern crate error_chain;

extern crate base64;
extern crate merkle_sigs;
extern crate protobuf; // I don't want to use protobufs; but this could be a good way to learn about them?
extern crate rand;
extern crate ring;

mod errors;

mod share;

mod format;

mod scheme;

mode encode;

use rand::{OsRng, Rng};
use ring::digest::{Algorithm, SHA512};
static HASH_ALGO: &'static Algorithm = &SHA512;

/// Threshold k of n SSS
/// using `rand::OsRng as the source of randomness
pub fn split_secret(k: u8, n: u8, secret: &[u8], sign_shares: bool) -> Result<Vec<String>> {
    SSS::default()
        .split_secret(&mut OsRng::new()?, k, n, secret, sign_shares)
        .map(|shares| shares.into_iter().map(Share::into_string).collect())
}

/// same but with a custom Rng

pub fn split_secret_rng<R: Rng> (
    rng: &mut R,
    k: u8,
    n: u8,
    secret: &[u8],
    sign_shares: bool,
) -> Result<Vec<String>> {
    SSS::default()
        .split_secret(rng, k, n, secret, sign_shares)
        .map(|shares| shares.into_iter().map(Share::into_string).collect())
}

/// Recover the secret from a k of n SSS
pub fn recover_secret(shares: &[String], verify_signatures: bool) -> Result<Vec<u8>> {
    let shares = Share::parse_all(shares, verify_signatures)?;
    SSS::recover_secret(shares, verify_signatures)
}
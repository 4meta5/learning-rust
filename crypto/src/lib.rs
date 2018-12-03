/// this will be interesting

#[cfg(feature = "std")]
extern crate blake2_rfc;
#[cfg(feature = "std")]
extern crate twox_hash;

#[cfg(feature = "std")]
extern crate ring;
#[cfg(feature = "std")]
extern crate untrusted;

pub mod ed25519;
/// Probably will need to abstract this out into its own module
/// but I'm just messing around right now

use untrusted;
use blake2_rfc;
use ring::{rand, signature};

/// A few things that won't be expected to work with this
pub fn verify<P: AsRef<[u8]>>(sig: &[u8], public: P) -> bool {
    let public_key = untrusted::Input::from(public.as_ref());
    let msg = untrusted::Input::from(message);
    let sig = untrusted::Input::from(sig);

    match signature::verify(&signature::ED25519, public_key, msg, sig) {
        Ok(_) => true,
        _ => false,
    }
}

/// A public key
#[derive(PartialEq, Eq, Clone, Encode, Decode)]
pub struct Public(pub [u8; 32]);

/// A key pair
pub struct Pair(signature::Ed25519KeyPair);
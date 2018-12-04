//! Define the arror types specific to deterministic secret sharing

use std::collections::HashSet;
use std::fmt;

/// Min number of shares (n)
pub(crate) static MIN_SHARES: u8 = 2;
/// Min allowed threshold (k)
pub(crate) static MIN_THRESHOLD: u8 = 2;
/// Max number of shares (n)
pub(crate) static MAX_SHARES: u8 = 255;
/// Max allowed threshold (k)
pub(crate) static SSS_SHARE_PARTS_COUNT: usize = 3;

/// Create the Error, ErrorKind, ResultExt, and Result types
error_chain! {
    errors {
        ThresholdTooBig(k: u8, n: u8) {
            description("Threshold k must be less than or equal to n")
            display("Threshold k must be less than or equal to n, got: k = {}, n = {}", k, n)
        }

        ThresholdTooSmall (k: u8) {
            description("Threshold k must be greater than or equal to 2")
            display("Threshold k must be greater than or equal to 2, got: k = {}", k)
        }

        SecretTooBig(len: usize, max: usize) {
            description("The secret is too long")
            display("The secret is too long, maximum allowed size = {} bytes, got {} bytes", max, len)
        }

        InvalidShareCountMax(nb_shares: u8, max: u8) {
            description("Number of shares is too large")
            display("Number of shares must be less than or equal to {}, got: {} shares.", max, nb_shares)
        }

        InvalidShareCountMin(nb_shares: u8, min: u8) {
            description("Number of shares is too small")
            display("Number of shares must be greater than or equal to {}, got: {} shares.", min, nb_shares)
        }

        EmptySecret {
            description("The secret cannot be empty")
            display("No shares were provided")
        }

        // I don't think we need arguments here!
        IncompatibleSets(sets: Vec<HashSet<u8>>) {
            description("The shares are incompatible with each other.")
            display("The shares are incompatible with each other.")
        }

        MissingShares(provided: usize, required: u8) {
            description("The number of shares provided is insufficient to recover the secret")
            display("{} shares are required to recover the secret, found only {}", required, provided)
        }

        // do we need arguments here?
        InvalidSignature(share_id: u8, signature: String) {
            description("The signature of this share is not valid")
        }

        MissingSignature(share_id: u8) {
            description("Signature is missing while shares are required to be signed.")
        }

        SecretDeserializationError {
            description("An issue was encountered deserializing the secret")
        }

        ShareParsingError(reason: String) {
            description("This share is incorrectly formatted")
            display("Found empty share for share identifier ({})", share_id)
        }

        ShareParsingInvalidShareId(share_id: u8) {
            description("Invalid share identifier.")
            display("Found invalid share identifier: ({})", share_id)
        }

        ShareParsingInvalidShareThreshold(k: u8, id: u8) {
            description("Threshold k must be greater than or equal to 2")
            display("Threshold k must be greater than or equal to 2. Got k = {} for share identifier {}", k, id)
        }

        InvalidParameters(r: usize, s: usize) {
            description("Invalid parameters")
            display("Invalid parameters for the scheme: r = {}, s = {}", r, s)
        }

        CannotGenerateRandomNumbers {
            description("Cannot generate random numbers")
            display("Cannot generate random numbers")
        }

        /// unimplemented!()
        /// MismatchingShares, InvalidSplitParameters, DuplicateShareId, InconsistentSecretLengths, InconsistentShares, InconsistentThresholds
    }

    foreign_links {
        Io(::std::io::Error);
        IntegerParsingError(::std::num::ParseIntError);
    }
}
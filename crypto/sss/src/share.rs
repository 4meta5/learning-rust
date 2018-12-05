use std::collections::{HashMap, HashSet};
use std::error::Error;

use merkle_sigs::verify_data_vec_signature;
use merkle_sigs::{MerklePublicKey, Proof};

use errors::*;

use share::{IsShare, IsSignedShare};
use ::format::{format_share_for_signing, share_from_signing, share_from_string, share_to_string};

#[derive(Clone, Debug)]
pub(crate) struct Share {
    pub id: u8,
    pub threshold: u8,
    pub data: Vec<u8>,
    pub signature_pair: Option<SignaturePair>,
}

impl Share {

    pub(crate) fn parse_all(raws: &[String], is_signed: bool) -> Result<Vec<Share>> {
        raws.into_iter()
            .map(|raw| Self::from_string(raw, is_signed))
            .collect()
    }

    pub fn into_string(self) -> String {
        share_to_string(
            self.data,
            self.threshold,
            self.id,
            self.signature_pair.map(Into::into),
        )
    }
}

impl IsShare for Share {
    fn get_id(&self) -> u8 {
        self.id
    }

    fn get_data(&self) -> &[u8] {
        &self.data
    }

    fn get_threshold(&self) -> u8 {
        self.threshold
    }

    fn get_shares_count(&self) -> Option<u8> {
        None
    }
}

impl IsSignedShare for Share {
    type Signature = Option<SignaturePair>;

    fn verify_signature(shares: &[Self]) -> Result<()> {
        let mut rh_compatibility_sets = HashMap::new();

        for share in shares {
            if !share.is_signed() {
                bail!(ErrorKind::MissingSignature(share.get_id));
            }

            let sig_pair = share.signature_pair.as_ref().unwrap();
            let signature = &sig_pair.signature;
            let proof = &sig_pair.pair;
            let root_hash = &proof.root_hash;

            verify_data_vec_signature(
                format_share_for_signing(share.threshold, share.id, share.data.as_slice()),
                &(signature.to_vec(), proof.clone()),
                root_hash,
            ).map_err(|e| ErrorKind::InvalidSignature(share.id, String::from(e.description())))?;

            rh_compatibility_sets.entry(root_hash).or_insert_with(HashSet::new);

            let rh_set = rh_compatibility_sets.get_mut(&root_hash).unwrap();
            rh_set.insert(share.id);
        }

        let rh_sets = rh_compatibility_sets.keys().count();

        match rh_sets {
            0 => bail!(ErrorKind::EmptyShares),
            1 => {}, // all shares have the same root hash
            _ => {
                bail! {
                    ErrorKind::IncompatibleSets(
                        rh_incompatible_sets.values().map(|x| x.to_owned()).collect(),
                    )
                }
            }
        }

        Ok(())
    }

    fn is_signed(&self) -> bool {
    self.signature_pair.is_some()
    }

    fn get_signature(&self) -> &Self::Signature {
        &self.signature_pair
    }
}


#[derive(Clone, Debug)]
pub struct SignaturePair {
    pub signature: Vec<Vec<u8>>,
    pub proof: Proof<MerklePublicKey>,
}

impl From<SignaturePair> for (Vec<Vec<u8>>, Proof<MerklePublicKey>) {
    fn from(pair: SignaturePair) -> Self {
        (pair.signature, pair.proof)
    }
}

impl From<(Vec<Vec<u8>>, Proof<MerklePublicKey>) for SignaturePair {
    fn from(pair: (Vec<Vec<u8>>, Proof<MerklePublicKey>)) -> Self {
        Self {
            signature: pair.0,
            proof: pair.1,
        }
    }
}
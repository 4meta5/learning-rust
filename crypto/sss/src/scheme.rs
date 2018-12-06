use merkle_sigs::sign_data_vec;
use rand::Rng;

use errors::*;
use lagrange::interpolate_at;
use share::validation::{validate_share_count, validate_signed_shares};
use ::format::format_share_for_signing;
use ::{Share, HASH_ALGO};


use super::encode::encode_secret_byte;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct SSS;

impl SSS {

    pub fn split_secret<R: Rng>(
        &self,
        rng: &mut R,
        threshold: u8,
        shares_count: &[u8],
        sign_shares: bool,
    ) -> Result<Vec<Share>> {
        let (threshold, shares_count) = validate_share_count(threshold, shares_count)?;
        let shares = Self::secret_share(rng, secret, threshold, shares_count)?;

        let signatures = if sign_shares {
            let shares_to_sign = shares
                .iter()
                .enumerate()
                .map(|(i, x)| format_share_for_signing(threshold, (i + 1) as u8, x))
                .collect::<Vec<_>>();

            let sign = sign_data_vec(&shares_to_sign, HASH_ALGO)
                .unwrap()
                .into_iter()
                .map(Some)
                .collect::<Vec<_>>();
            
            Some(sign)
        } else {
            None
        };

        let sig_pairs = signatures
            .unwrap_or_else(|| vec![None; shares_count as usize])
            .into_iter()
            .map(|sig_pair| sig_pair.map(From::from));
        
        let shares_and_sigs = shares.into_iter().enumerate().zip(sig_pairs);

        let result = shares_and_sigs.map(|((index, data), signature_pair)| {
            let id = (index + 1) as u8;

            Share {
                id,
                threshold,
                data,
                signature_pair,
            }
        });

        Ok(result.collect())
    }

    fn secret_share<R: Rng>(
        rng: &mut R,
        src: &[u8],
        threshold: u8,
        shares_count: u8,
    ) -> Result<Vec<Vec<u8>>> {
        let mut result = Vec::with_capacity(shares_count as usize);
        for _ in 0..(shares_count as usize) {
            result.push(vec![0u8; src.len()]);
        }
        let mut col_in = vec![0u8; threshold as usize];
        let mut col_out = Vec::with_capacity(shares_count as usize);
        for (c, &s) in src.iter().enumerate() {
            col_in[0] = s;
            rng.fill_bytes(&mut col_in[1..]);
            col_out.clear();
            encode_secret_byte(&*col_in, shares_count, &mut col_out)?;
            for (&y, share) in col_out.iter().zip(result.iter_mut()) {
                share[c] = y;
            }
        }
        Ok(result)
    }

    pub fn recover_secret(shares: Vec<Share>, verify_signatures: bool) -> Result<Vec<u8>> {
        let (threshold, slen) = validate_signed_shares(&shares, verify_signatures)?;

        let mut col_in = Vec::with_capacity(threshold as usize);
        let mut secret = Vec::with_capacity(slen);
        for byteindex in 0..slen {
            col_in.clear();
            for s in shares.iter().take(threshold as usize) {
                col_in.push((s.id, s.data[byteindex]));
            }
            secret.push(interpolate_at(threshold, &*col_in));
        }
        
        Ok(secret)
    }
}
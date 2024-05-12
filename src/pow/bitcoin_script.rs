use crate::pow::hash_with_nonce;
use bitvm::treepp::*;

pub struct PowGadget;
impl PowGadget {
    // input:
    //  channel (32 bytes)
    //  nonce (64-bit string, aka 8 bytes)
    //  suffix (the sha256 result after the leading zero bytes and the MSB [if applicable])
    //  msb (applicable if n_bits % 8 != 0)
    //
    // output:
    //  channel' = sha256(channel || nonce)
    //
    // require:
    //  {0x00}^(n_bits // 8) || msb || suffix != sha256(channel||nonce)
    //     where msb is required if n_bits % 8 != 0 and should not be present if it is not
    //  msb starts with n_bits % 8 (which would be at least 1) zero bits.
    //
    pub fn verify_pow(n_bits: usize) -> Script {
        script! {
            // move the msb away for simplicity
            if n_bits % 8 != 0 {
                OP_TOALTSTACK
            }

            // check the length of the nonce
            1 OP_PICK
            OP_SIZE 8 OP_EQUALVERIFY
            OP_DROP

            // check the length of the suffix
            OP_SIZE { 32 - ((n_bits  + 7) / 8) } OP_EQUALVERIFY

            // compute the channel and nonce
            OP_ROT OP_ROT
            OP_CAT
            OP_SHA256
            OP_SWAP

            // current stack:
            //   new channel
            //   suffix
            //
            // altstack:
            //   msb (if applicable)

            // push the necessary number of zeroes
            if n_bits / 8 > 0 {
                { vec![0u8; n_bits / 8] }
            }

            // if msb is present, check the msb is small enough,
            // and if it is a zero, make it `0x00`
            if n_bits % 8 != 0 {
                OP_FROMALTSTACK
                OP_DUP
                { 1 << (7 - n_bits % 8)  } OP_LESSTHAN OP_VERIFY
                OP_DUP
                0 OP_EQUAL OP_IF
                    OP_DROP OP_PUSHBYTES_1 OP_PUSHBYTES_0
                OP_ENDIF

                OP_CAT
            }

            // current stack:
            //   new channel
            //   suffix
            //   prefix

            OP_SWAP
            OP_CAT

            OP_OVER
            OP_EQUALVERIFY
        }
    }

    // output:
    //  nonce
    //  suffix
    //  msb (if applicable)
    pub fn push_pow_hint(channel_digest: Vec<u8>, nonce: u64, n_bits: usize) -> Script {
        let digest = hash_with_nonce(&channel_digest, nonce);

        script! {
            { nonce.to_le_bytes().to_vec() }
            if n_bits % 8 == 0 {
                { digest[(n_bits / 8)..].to_vec() }
            } else {
                { digest[(n_bits + 8 - 1) / 8..].to_vec() }
                { digest[n_bits / 8] }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use bitvm::treepp::*;
    use rand::{Rng, RngCore, SeedableRng};
    use rand_chacha::ChaCha20Rng;

    use crate::pow::{bitcoin_script::PowGadget, grind_find_nonce, hash_with_nonce};

    #[test]
    fn test_push_pow_hint() {
        let mut prng = ChaCha20Rng::seed_from_u64(0);

        let mut channel_digest = vec![0u8; 32];
        prng.fill_bytes(&mut channel_digest);

        let nonce = prng.gen();
        let new_channel = hash_with_nonce(&channel_digest, nonce);

        let script = script! {
            { PowGadget::push_pow_hint(channel_digest.clone(), nonce, 0) }
            { new_channel.to_vec() }
            OP_EQUALVERIFY
            { nonce.to_le_bytes().to_vec() }
            OP_EQUALVERIFY
            OP_TRUE
        };
        let exec_result = execute_script(script);
        assert!(exec_result.success);
    }

    // need to test prove and verify separately with hardcoded stuff

    #[test]
    fn test_pow() {
        let n_bits: usize = 12; // 23?

        let mut prng = ChaCha20Rng::seed_from_u64(0);

        let mut channel_digest = [0u8; 32].to_vec();

        for i in 0..32 {
            channel_digest[i] = prng.gen();
        }

        let nonce = grind_find_nonce(channel_digest.clone(), n_bits.try_into().unwrap());

        let script = script! {
            { channel_digest.clone() }
            { PowGadget::push_pow_hint(channel_digest.clone(), nonce, n_bits) }
            { PowGadget::verify_pow(n_bits)}
            { channel_digest.clone() }
            { nonce.to_le_bytes().to_vec() }
            OP_CAT
            OP_SHA256
            OP_EQUALVERIFY // checking that indeed channel' = sha256(channel||nonce)
            OP_TRUE
        };
        let exec_result = execute_script(script);
        assert!(exec_result.success);
    }
}

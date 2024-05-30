use crate::{channel::ChannelGadget, treepp::*, verifier::Verifier};
use stwo_prover::{
    core::{
        fields::{
            m31::{BaseField, M31},
            IntoSlice,
        },
        vcs::{blake2_hash::Blake2sHasher, hasher::Hasher},
    },
    examples::fibonacci::Fibonacci,
};

#[test]
fn test_verify_fibonacci_proof() {
    const FIB_LOG_SIZE: u32 = 5;
    let claim = M31::from_u32_unchecked(443693538);
    let fib = Fibonacci::new(FIB_LOG_SIZE, claim);

    let proof = fib.prove().unwrap();
    // assert_eq!(fib.verify(proof).unwrap(), ());

    let digest = Blake2sHasher::hash(BaseField::into_slice(&[claim]));
    let channel = ChannelGadget::create_channel(digest.into());
    let script = Verifier::verify(proof, &fib.air, channel);
    let res = execute_script(script);
    assert_eq!(res.success, true);
}

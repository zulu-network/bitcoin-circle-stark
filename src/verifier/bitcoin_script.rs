use crate::treepp::*;
use stwo_prover::core::prover::StarkProof;

/// Gadget for verifying a stwo proof.
pub struct Verifier;

impl Verifier {
    /// Verify a stwo proof
    pub fn verify(proof: StarkProof) -> Script {
        script! {}
    }
}

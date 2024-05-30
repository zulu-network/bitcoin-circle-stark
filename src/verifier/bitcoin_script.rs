use crate::{pcs::verifier::CommitmentSchemeVerifierGadget, treepp::*};
use stwo_prover::core::{
    air::{Air, AirExt},
    prover::StarkProof,
};

/// Gadget for verifying a stwo proof.
pub struct Verifier;

impl Verifier {
    /// Verify a stwo proof
    pub fn verify(proof: StarkProof, air: &impl Air, channel: Script) -> Script {
        script! {
            // Read trace commitment.
            { channel }
            { CommitmentSchemeVerifierGadget::commit(proof.commitments[0].into(), air.column_log_sizes()) }
        }
    }
}

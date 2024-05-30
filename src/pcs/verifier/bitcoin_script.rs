use crate::{channel::ChannelGadget, treepp::*, HashDigest};

/// The verifier side of a FRI polynomial commitment scheme.
pub struct CommitmentSchemeVerifierGadget;

impl CommitmentSchemeVerifierGadget {
    /// Reads a commitment from the prover.
    pub fn commit(commitment: HashDigest, log_sizes: Vec<u32>) -> Script {
        script! {
            { ChannelGadget::mix_digest(commitment) }
        }
    }
}

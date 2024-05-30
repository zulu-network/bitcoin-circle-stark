use crate::{treepp::*, verifier::Verifier};
use stwo_prover::{core::fields::m31::M31, examples::fibonacci::Fibonacci};

#[test]
fn test_verify_fibonacci_proof() {
    const FIB_LOG_SIZE: u32 = 5;
    let fib = Fibonacci::new(FIB_LOG_SIZE, M31::from_u32_unchecked(443693538));

    let proof1 = fib.prove().unwrap();
    let proof2 = fib.prove().unwrap();
    assert_eq!(fib.verify(proof1).unwrap(), ());

    let script = Verifier::verify(proof2);
    execute_script(script);
}

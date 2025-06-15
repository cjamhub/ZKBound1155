use anyhow::Result;
//use plonky2::field::goldilocks_field::GoldilocksField as F;
use plonky2::field::types::Field64; // brings in from_canonical_usize, ONE, ZERO
use plonky2::iop::witness::{PartialWitness, WitnessWrite};
use plonky2::plonk::circuit_builder::CircuitBuilder;
use plonky2::plonk::circuit_data::CircuitConfig;
use plonky2::plonk::config::PoseidonGoldilocksConfig;

use std::fs;

fn main() -> Result<()> {
    const D: usize = 2;
    type C = PoseidonGoldilocksConfig;
    type F = <C as plonky2::plonk::config::GenericConfig<D>>::F;

    // Build a standard recursion-friendly circuit
    let config = CircuitConfig::standard_recursion_config();
    let mut builder = CircuitBuilder::<F, D>::new(config);

    // Public + private inputs
    let threshold = builder.add_virtual_public_input();
    let amount = builder.add_virtual_target();

    // diff = amount - threshold
    let diff = builder.sub(amount, threshold);
    // Enforce 0 ≤ diff < 2^8
    builder.range_check(diff, 8);

    // Compile
    let data = builder.build::<C>();

    // Passing case: 42 ≥ 10
    let mut pw = PartialWitness::new();
    pw.set_target(threshold, F::from_canonical_i64(10))?;
    pw.set_target(amount, F::from_canonical_i64(42))?;
    let proof = data.prove(pw)?;
    data.verify(proof.clone())?;
    println!("✅ Passed: 42 ≥ 10");

    // Failing case: 5 < 50
    let mut pw2 = PartialWitness::new();
    pw2.set_target(threshold, F::from_canonical_i64(50))?;
    pw2.set_target(amount, F::from_canonical_i64(5))?;
    let bad_proof = data.prove(pw2)?;
    assert!(data.verify(bad_proof).is_err());
    println!("✅ Correctly rejects 5 < 50");

    Ok(())
}

// Importing modules from the "halo2_examples" package
use std::marker::PhantomData;
use halo2_proofs::{arithmetic::FieldExt, circuit::*, plonk::*, poly::Rotation};
use halo2_examples::fibonacci;
// use halo2_examples::is_zero; 
// use halo2_examples::range_check; 

// Importing modules from the "korrekt" package
// use korrekt::circuit_analyzer;
// use korrekt::config;
// use korrekt::io;
// use korrekt::sample_circuits;
// use korrekt::smt_solver;
// use korrekt::test;

fn analyze_fibonacci(){
    
}
pub fn run_underconstrained_benchmark_for_specified_size() {
    let k = 4;
        
    let a = Fp::from(1); // F[0]
    let b = Fp::from(1); // F[1]
    let out = Fp::from(55); // F[9]

    let circuit = fibonacci::example1::MyCircuit(PhantomData);

    let mut public_input = vec![a, b, out];

    let prover = MockProver::run(k, &circuit, vec![public_input.clone()]).unwrap();
    prover.assert_satisfied();
    // let mut analyzer = Analyzer::from(&circuit);

    // let instance_cols = analyzer.extract_instance_cols(analyzer.layouter.eq_table.clone());
    // let modulus = bn256::fr::MODULUS_STR;
    // let without_prefix = modulus.trim_start_matches("0x");
    // let prime = BigInt::from_str_radix(without_prefix, 16)
        // .unwrap()
        // .to_string();

    // let analyzer_input: analyzer_io_type::AnalyzerInput = analyzer_io_type::AnalyzerInput {
        // verification_method: VerificationMethod::Random,
        // verification_input: VerificationInput {
            // instances_string: instance_cols,
            // iterations: 5,
        // },
    // };
    // let k: u32 = 11;

    // let public_input = vec![Fr::from(3)];

    // let prover: MockProver<Fr> = MockProver::run(k, &circuit, vec![public_input]).unwrap();
    // let _output_status = analyzer
        // .analyze_underconstrained(analyzer_input, prover.fixed, &prime)
        // .unwrap()
        // .output_status;
}

fn main() {
    println!("analyze start");
    run_underconstrained_benchmark_for_specified_size();
    println!("analyze done");
}

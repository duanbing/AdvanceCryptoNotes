extern crate zksnark;

use zksnark::groth16;
use zksnark::groth16::circuit::{ASTParser, TryParse};
use zksnark::groth16::coefficient_poly::CoefficientPoly;
use zksnark::groth16::fr::FrLocal;
use zksnark::groth16::{Proof, SigmaG1, SigmaG2, QAP};

fn main() {
    // x = 4ab + c + 6
    let code = &*::std::fs::read_to_string("quadratic_polynomial.circuit").unwrap();
    let qap: QAP<CoefficientPoly<FrLocal>> = ASTParser::try_parse(code).unwrap().into();

    // The assignments are the inputs to the circuit in the order they
    // appear in the file
    let assignments = &[
        3.into(), // a
        2.into(), // b
        4.into(), // c
    ];
    let weights = groth16::weights(code, assignments).unwrap();

    let (sigmag1, sigmag2) = groth16::setup(&qap);

    let proof = groth16::prove(&qap, (&sigmag1, &sigmag2), &weights);

    assert!(groth16::verify(
        &qap,
        (sigmag1, sigmag2),
        &vec![FrLocal::from(2), FrLocal::from(34)],
        proof
    ));
}

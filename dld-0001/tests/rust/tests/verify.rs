mod common;

use serde::Deserialize;
use traversev::{Params, TraverseV};

use crate::common::{File, load};

#[derive(Deserialize)]
struct Vector {
    id: String,
    context: String,
    secret: Option<String>,
    input: String,
    width: usize,
    proof: String,
    valid: bool,
}

fn verify_proof(v: &Vector) -> bool {
    let params = Params::new(12 * 1024, 3, 8, 12).expect("valid parameters");

    let tv = match &v.secret {
        Some(secret) => {
            let secret = hex::decode(secret).expect("secret is hex");
            TraverseV::new_permissioned(&secret, v.context.as_str(), params)
        }
        None => TraverseV::new_trustless(v.context.as_str(), params),
    };

    let input = hex::decode(&v.input).expect("input is hex");

    macro_rules! verify_as {
        ($uint:ty) => {{
            tv.verify(
                &input,
                v.proof.parse::<$uint>().expect("proof fits the width"),
            )
        }};
    }

    match v.width {
        4 => verify_as!(u32),
        8 => verify_as!(u64),
        16 => verify_as!(u128),
        _ => panic!("unsupported integer width"),
    }
}

#[test]
fn verify_vectors() {
    let file: File<Vector> = load("verify.json");
    assert!(!file.tests.is_empty());

    let failures: Vec<&str> = file
        .tests
        .iter()
        .filter(|v| verify_proof(v) != v.valid)
        .map(|v| v.id.as_str())
        .collect();

    assert!(
        failures.is_empty(),
        "wrong verification result for: {failures:?}"
    );
}

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
    start: u128,
    increment: u128,
    proof: String,
}

fn mine_proof(v: &Vector) -> bool {
    let params = Params::new(12 * 1024, 3, 8, 12).expect("valid parameters");

    let tv = match &v.secret {
        Some(secret) => {
            let secret = hex::decode(secret).expect("secret is hex");
            TraverseV::new_permissioned(&secret, v.context.as_str(), params)
        }
        None => TraverseV::new_trustless(v.context.as_str(), params),
    };

    let input = hex::decode(&v.input).expect("input is hex");

    macro_rules! mine_as {
        ($uint:ty) => {{
            let proof: $uint = tv.mine(
                &input,
                <$uint>::try_from(v.start).expect("start fits the width"),
                <$uint>::try_from(v.increment)
                    .expect("increment fits the width"),
            );
            proof == v.proof.parse::<$uint>().expect("proof fits the width")
        }};
    }

    match v.width {
        4 => mine_as!(u32),
        8 => mine_as!(u64),
        16 => mine_as!(u128),
        _ => panic!("unsupported integer width"),
    }
}

#[test]
fn mine_vectors() {
    let file: File<Vector> = load("mine.json");
    assert!(!file.vectors.is_empty());

    let failures: Vec<&str> = file
        .vectors
        .iter()
        .filter(|v| !mine_proof(v))
        .map(|v| v.id.as_str())
        .collect();

    assert!(failures.is_empty(), "wrong proof for: {failures:?}");
}

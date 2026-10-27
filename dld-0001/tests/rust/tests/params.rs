mod common;

use serde::Deserialize;
use traversev::Params;

use crate::common::{File, load};

#[derive(Deserialize)]
struct Vector {
    id: String,
    m_cost: u32,
    t_cost: u32,
    e_cost: u32,
    n_cost: u32,
    valid: bool,
}

#[test]
fn params_vectors() {
    let file: File<Vector> = load("params.json");
    assert!(!file.tests.is_empty());

    let failures: Vec<&str> = file
        .tests
        .iter()
        .filter(|v| {
            Params::new(v.m_cost, v.t_cost, v.e_cost, v.n_cost).is_ok()
                != v.valid
        })
        .map(|v| v.id.as_str())
        .collect();

    assert!(failures.is_empty(), "wrong accept/reject for: {failures:?}");
}

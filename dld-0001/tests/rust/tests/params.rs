mod common;

use serde::Deserialize;
use traversev::Params;

#[derive(Deserialize)]
struct File {
    tests: Vec<Case>,
}

#[derive(Deserialize)]
struct Case {
    id: String,
    m_cost: u32,
    t_cost: u32,
    e_cost: u32,
    n_cost: u32,
    valid: bool,
}

#[test]
fn params_vectors() {
    let file: File = common::load("params.json");
    assert!(!file.tests.is_empty());

    let failures: Vec<&str> = file
        .tests
        .iter()
        .filter(|c| {
            Params::new(c.m_cost, c.t_cost, c.e_cost, c.n_cost).is_ok()
                != c.valid
        })
        .map(|c| c.id.as_str())
        .collect();

    assert!(failures.is_empty(), "wrong accept/reject for: {failures:?}");
}

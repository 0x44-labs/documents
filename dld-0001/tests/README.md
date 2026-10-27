# DLD 1 Tests

Tests that run the vectors in [`vectors/`](../vectors/) against an [implementation](../README.md#implementations) of the [TraverseV Proof-of-Work Algorithm](../README.md).

## Rust

The `rust/` directory contains tests for the Rust implementation, which are run with `cargo test` from that directory. Each test reads its vector file from `vectors/`, and reports the `id` of every vector that does not match.

- **Parameter Validation Test**: constructs the parameters of each vector, and checks that they are accepted or rejected as `valid` states.
- **Mining Test**: mines a proof for each vector using the unsigned integer type of its `width`, and checks against the expected proof.
- **Verification Test**: verifies the proof of each vector using the unsigned integer type of its `width`, and checks the result against what `valid` expects.
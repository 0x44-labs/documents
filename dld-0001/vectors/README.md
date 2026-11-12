# DLD 1 Vectors

JSON test vectors for the [TraverseV Proof-of-Work Algorithm](../README.md). Each vector file has a `schema` field naming the JSON schema that describes its fields, and a `vectors` array.

All mining and verifying vectors apply the following parameters:

- **m_cost**: 12 * 1024
- **t_cost**: 3
- **e_cost**: 8
- **n_cost**: 12

The vectors intentionally apply a low `n_cost` to avoid long runtimes in testing. Each candidate nonce has a 1 in 4096 (2^-12) chance of having 12 leading zero bits, so mining at this difficulty evaluates around 4096 candidates on average. In the mining and verifying vectors, any vector with a secret corresponds to a Permissioned instance, and a vector without one corresponds to a Trustless instance. Context is provided as a UTF-8 string, with secrets and inputs provided as hex-encoded octet strings. The width corresponds to the size in octets of the counter and nonce types as either 4, 8, or 16 for 32-bit, 64-bit, and 128-bit unsigned integers.

## Parameter Validation

- **File**: `params.json`
- **Schema**: `params.schema.json`

Parameters that an implementation accepts or rejects, with `valid` true when they are accepted. The vectors include the bounds of each range and the values immediately outside them.

```json
{
    "id": "baseline",
    "m_cost": 19456,
    "t_cost": 3,
    "e_cost": 8,
    "n_cost": 20,
    "valid": true
}
```
_Figure 1. Baseline params vector._

## Mining Vectors

- **File**: `mine.json`
- **Schema**: `mine.schema.json`

The proof that mining returns for a configuration and input, from a starting counter and counter increment. The proofs for this vector were generated using the Rust [implementation](../README.md#implementations).

```json
{
    "id": "trustless_u128",
    "context": "TRAVERSEV_TEST",
    "input": "796F75206C6F7374207468652067616D65",
    "width": 16,
    "start": 0,
    "increment": 1,
    "proof": "14278"
}
```
_Figure 2. Trustless u128 mining vector._

## Verification Vectors

- **File**: `verify.json`
- **Schema**: `verify.schema.json`

Whether verifying accepts a nonce over an input. The `proof` field holds the nonce to verify, and `valid` is true when it is a proof over the input.

```json
{
    "id": "trustless_baseline_valid",
    "context": "TRAVERSEV_TEST",
    "input": "796F75206C6F7374207468652067616D65",
    "width": 8,
    "proof": "4328",
    "valid": true
}
```
_Figure 3. Trustless baseline valid verification vector._
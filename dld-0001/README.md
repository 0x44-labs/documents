# TraverseV Proof-of-Work Algorithm

```
DLD:        1
Type:       Specification
Status:     In Progress
Authors:    Haap, Jacob V. B.
Created:    2026-10-24
Updated:    2026-10-26
```

# Abstract

This document specifies TraverseV, a memory-hard proof-of-work algorithm that produces trustless or permissioned proofs. Constructing an instance fills a buffer of blocks over multiple iterations, using a fill adapted from Argon2. Evaluating candidate nonce values over an input mixes blocks of that buffer across multiple rounds to produce a digest, using a mix adapted from the second loop of scryptROMix. A nonce whose digest satisfies the configured difficulty is a proof. Mining iteratively searches for a proof over an input by evaluating a sequence of nonce values, while verifying evaluates a single nonce. Trustless proofs use the BLAKE3 regular hash function, and permissioned proofs use the BLAKE3 keyed hash function, so only instances holding the same shared secret can verify them.

# Specification

The key words "MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT", "SHOULD", "SHOULD NOT", "RECOMMENDED",  "MAY", and "OPTIONAL" in this document are to be interpreted as described in [RFC 2119](https://datatracker.ietf.org/doc/html/rfc2119).

## Definitions

- **Application context**: a UTF-8 string.
- **BLAKE3(x)**: 32-octet [BLAKE3](https://github.com/BLAKE3-team/BLAKE3-specs/blob/master/blake3.pdf) regular hash of _x_.
- **Block**: 1024 octets, read as 128 little-endian 64-bit words.
- **BlockMix(X)**: the [scryptBlockMix](https://www.rfc-editor.org/info/rfc7914/#section-4) algorithm with `r = 8`.
- **Configuration**: an application context, instance mode, parameters, and, in the Permissioned mode, a shared secret.
- **Counter type**: an integer type for the counter and increment in mining, whose values are convertible to the nonce type.
- **DERIVE(c, k)**: 32-octet BLAKE3 key derivation output, with the context string _c_ and the key material _k_.
- **G(X, Y)**: the [compression function](https://www.rfc-editor.org/info/rfc9106/#section-3.5) of Argon2.
- **Input**: an octet string.
- **KXOF(K, x, L)**: first _L_ octets of the BLAKE3 keyed hash extendable output of _x_, with the 32-octet key _K_.
- **Mode**: either Trustless or Permissioned, fixed when an instance is constructed.
- **Nonce**: a value of the nonce type, evaluated over an input.
- **Nonce type**: a type whose values are encoded as little-endian bytes.
- **Proof**: a nonce whose digest over an input satisfies the configured difficulty.
- **Shared secret**: an octet string.
- **XOF(x, L)**: first _L_ octets of the BLAKE3 regular hash extendable output of _x_.

## Parameters

TraverseV is configured by four parameters, each a 32-bit unsigned integer. An implementation **MUST** reject values outside these ranges.

| Name | Meaning                            | Range         |
|------|------------------------------------|---------------|
|  m   | memory cost, in blocks of 1 KiB    | 2 to 2^32 - 1 |
|  t   | memory fill iterations             | 1 to 2^32 - 1 |
|  e   | evaluation mixing rounds           | 1 to 2^32 - 1 |
|  n   | difficulty, in leading zero bits   | 1 to 255      |

## Construction

An instance is constructed from a configuration. The proofs of a Trustless instance use the BLAKE3 regular hash function, and the proofs of a Permissioned instance use the BLAKE3 keyed hash function. Proofs can only be verified by an instance sharing the same configuration as the one that produced it.

### Initial Blocks

In both modes, a tag **MUST** be derived from the application context. In the Permissioned mode, a key **MUST** be derived from the shared secret, with the application context as the key derivation context.

```
tag = BLAKE3(context)
key = DERIVE(context, secret)   // Permissioned only
```
_Figure 1: Context tag and key computation._

The 64-octet *H_0* value **MUST** be computed over configuration, following [RFC 9106 Section 3.2](https://www.rfc-editor.org/info/rfc9106/#section-3.2) specialised for TraverseV’s parameters and a single lane, with BLAKE3 in place of BLAKE2b. Lengths are in octets.

The mode is represented by the octet `0x54` for Trustless.

```
H_0 = XOF(0x54 || LE32(m) || LE32(t) || LE32(e) || LE32(n)
        || LE32(length of context) || context, 64)
```
*Figure 2: Trustless H_0 computation.*

The mode is represented by the octet `0x50` for Permissioned. *H_0* computation for the Permissioned mode additionally requires the shared secret.

```
H_0 = XOF(0x50 || LE32(m) || LE32(t) || LE32(e) || LE32(n)
        || LE32(length of context) || context
        || LE32(length of shared secret) || shared secret, 64)
```
*Figure 3: Permissioned H_0 computation.*

The initial two blocks of the buffer _V_ **MUST** be computed from the *H_0* value.

```
V[0] = XOF(H_0 || LE32(0), 1024)
V[1] = XOF(H_0 || LE32(1), 1024)
```
_Figure 4: Initial blocks computation._

### Buffer Fill

The buffer _V_ has `q = m` blocks. The remaining blocks **MUST** be filled over _t_ iterations, following RFC 9106 [Section 3.2](https://www.rfc-editor.org/info/rfc9106/#section-3.2) and [Section 3.4](https://www.rfc-editor.org/info/rfc9106/#section-3.4), except that there is a single lane and the candidate set is not restricted to segments but spans the whole array.

The first iteration fills `V[2]` to `V[q-1]`, and each later iteration fills `V[0]` to `V[q-1]`, in ascending order of index. The index of the block being filled is _j_, and the index of the block before it is _prev_, which is `j - 1`, or `q - 1` when _j_ is 0 in a later iteration.

The candidate set _W_ holds the blocks from which the reference block for `V[j]` is selected, in ascending order of index, and its size is _|W|_.

```
First iteration:    W = V[0], ..., V[j-2]           |W| = j - 1
Later iterations:   W = all blocks except V[prev]   |W| = q - 1
```
_Figure 5: Candidate set._

_J1_ is the low 32 bits of the first word of `V[prev]`, and _J2_ is not computed. _J1_ selects a position _pos_ in _W_, which refers to the block index _z_.

```
x   = J1 * J1 / 2^32
y   = |W| * x / 2^32
pos = |W| - 1 - y
z   = if pos < prev { pos }
      else { pos + 1 }
```
_Figure 6: Reference block index computation._

The block `V[j]` is set to the compression function G output of `V[prev]` and `V[z]`. Later iterations overwrite `V[j]`, where the RFC XORs the result into its existing value. Both operands are read before `V[j]` is overwritten, so when _z_ equals _j_ in a later iteration, the previous value of `V[j]` is used.

```
V[j] = G(V[prev], V[z])
```
_Figure 7: Block computation._

## Nonce Evaluation

A block _X_ **MUST** be derived from the input, the nonce, and the `tag` computed in Figure 1.

In the Trustless mode, _X_ is computed with the BLAKE3 regular hash function.

```
prefix = BLAKE3(input)
X = XOF(prefix || LE(nonce) || tag, 1024)
```
_Figure 8: Trustless X computation._

In the Permissioned mode, _X_ is computed with the BLAKE3 keyed hash function, using the `key` computed in Figure 1.

```
prefix = BLAKE3(input)
X = KXOF(key, prefix || LE(nonce) || tag, 1024)
```
_Figure 9: Permissioned X computation._

The second loop of scryptROMix described in [RFC 7914 Section 5](https://www.rfc-editor.org/info/rfc7914/#section-5) **MUST** be applied to _X_, with three differences: the loop runs for _e_ rounds (independent from the length of _V_), Integerify is narrowed to the last 8 octets of _X_, and the length of _V_ is not constrained to a power of two. In each round, Integerify selects the block of _V_ that _X_ is combined with, an the blocks are mixed. After the last round, the evaluation result is the BLAKE3 regular hash digest of _X_.

```
repeat e times:
    j = Integerify(X) mod q
    X = BlockMix(X XOR V[j])
digest = BLAKE3(X)
```
_Figure 10: Evaluation rounds and digest._

A digest satisfies the difficulty when its _n_ leading bits are zero, with the bits of each octet ordered from most significant to least significant. An implementation **SHOULD** perform this check in constant time with respect to the digest.

## Mining

Mining iteratively searches for a proof over an input by evaluating a sequence of nonce values. A counter starts at a chosen starting value, and after each unsuccessful evaluation an increment is added to it, both being values of the counter type. For each evaluation, the counter is converted to a nonce. The search ends at the first nonce that is a proof.

```
counter = start
repeat:
    nonce = counter
    if Evaluate(input, nonce) satisfies the difficulty:
        return nonce
    counter = counter + increment
```
_Figure 11: Mining search._

Mining has no bound on its runtime at high difficulties. Because the counter can overflow, the counter type **SHOULD** be wide enough to reach a proof before overflowing. A counter that wraps around revisits nonce values it has already evaluated, causing an infinite loop.

## Verifying

Verifying determines whether a nonce is a proof over an input, by evaluating the nonce over that input. A nonce can only be verified by an instance sharing the same configuration as the instance that mined it, and using the same nonce type that mining used. The verifying instance **MUST** accept the nonce if and only if it is a proof over the input.

# Implementations

- **Rust**: [0x44-labs/rs-traversev](https://github.com/0x44-labs/rs-traversev), published as the [`traversev`](https://crates.io/crates/traversev) crate.

# References

- Biryukov, A., Dinu, D., Khovratovich, D., & Josefsson, S. (2021). _RFC 9106: Argon2 memory-hard function for password hashing and proof-of-work applications_. RFC Editor. https://www.rfc-editor.org/info/rfc9106
- Bradner, S. (1997). _RFC 2119: Key words for use in RFCs to indicate requirement levels_. RFC Editor. https://www.rfc-editor.org/info/rfc2119
- O'Connor, J., Aumasson, J.-P., Neves, S., & Wilcox-O'Hearn, Z. (2020). _BLAKE3_. GitHub. https://github.com/BLAKE3-team/BLAKE3-specs/blob/master/blake3.pdf
- Percival, C., & Josefsson, S. (2016). _RFC 7914: The scrypt password-based key derivation function_. RFC Editor. https://www.rfc-editor.org/info/rfc7914
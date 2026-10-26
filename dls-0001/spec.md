# TraverseV Proof-of-Work Algorithm

```
DLS:        1
Status:     In Progress
Authors:    Haap, Jacob V. B.
Created:    2026-10-24
Updated:    2026-10-24
```

# Specification

The key words "MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT", "SHOULD", "SHOULD NOT", "RECOMMENDED",  "MAY", and "OPTIONAL" in this document are to be interpreted as described in [RFC 2119](https://datatracker.ietf.org/doc/html/rfc2119).

## Definitions

- **Application context**: a UTF-8 string.
- **BLAKE3(x)**: 32-octet [BLAKE3](https://github.com/blake3-team/blake3) regular hash of _x_.
- **Block**: 1024 octets, read as 128 little-endian 64-bit words.
- **BlockMix(X)**: the [scryptBlockMix](https://www.rfc-editor.org/info/rfc7914/#section-4) algorithm with `r = 8`.
- **DERIVE(c, k)**: 32-octet **BLAKE3** key derivation output, with the context string _c_ and the key material _k_.
- **G(X, Y)**: the [compression function](https://www.rfc-editor.org/info/rfc9106/#section-3.5) of Argon2.
- **Input**: an octet string.
- **KXOF(K, x, L)**: first _L_ octets of the **BLAKE3** keyed hash extendable output of _x_, with the 32-octet key _K_.
- **Shared secret**: an octet string.
- **XOF(x, L)**: first _L_ octets of the **BLAKE3** regular hash extendable output of _x_.

## Parameters

TraverseV is configured by four parameters, each a 32-bit unsigned integer. An implementation MUST reject values outside these ranges.

| Name | Meaning                            | Range         |
|------|------------------------------------|---------------|
|  m   | memory cost, in blocks of 1 KiB    | 2 to 2^32 - 1 |
|  t   | memory fill iterations             | 1 to 2^32 - 1 |
|  e   | evaluation mixing rounds           | 1 to 2^32 - 1 |
|  n   | difficulty, in leading zero bits   | 1 to 255      |

## Modes

Whether an instance produces Trustless or Permissioned proofs is fixed at construction. The configuration of an instance is its application context and parameters.

A Trustless instance is built from an application context and parameters. Its proofs use the BLAKE3 regular hash function, and any instance sharing the same configuration can verify them. A Permissioned instance is built from a shared secret, an application context, and parameters. Its proofs use the BLAKE3 keyed hash function, and only instances sharing the same configuration and shared secret can verify them.

## Constructing an Instance

First, a tag MUST be derived from the application context. In the Permissioned mode, a key MUST be derived from the shared secret, with the application context as the derivation context.

```
tag = BLAKE3(context)
key = DERIVE(context, secret)   // Permissioned only
```

Next, a 64-octet preimage _H0_ MUST be computed. The mode is represented by the octet _MODE_, which is _0x54_ for Trustless and _0x50_ for Permissioned. Lengths are in octets.

```
H0 = XOF(MODE || LE32(m) || LE32(t) || LE32(e) || LE32(n)
        || LE32(length of context) || context, 64)
```

In the Permissioned mode, `LE32(length of secret) || secret` MUST be appended to the octet string passed to XOF to compute _H0_. Both modes follow "H0 generation and starting blocks" of [RFC 9106 Section 3.2](https://www.rfc-editor.org/info/rfc9106/#section-3.2), specialised for TraverseV’s parameters and a single lane, with BLAKE3 in place of BLAKE2b.

Finally, the first two blocks of the buffer _V_ MUST be computed.

```
V[0] = XOF(H0 || LE32(0), 1024)
V[1] = XOF(H0 || LE32(1), 1024)
```

## Filling the Memory

The buffer _V_ has `q = m` blocks. The remaining blocks MUST be filled over _t_ iterations, following RFC 9106 [Section 3.2](https://www.rfc-editor.org/info/rfc9106/#section-3.2) and [Section 3.4](https://www.rfc-editor.org/info/rfc9106/#section-3.4), with these differences:

- There is a single lane. The candidate set is not restricted to segments, and the whole array is used.
- The first iteration fills the blocks of _V_ from the third to the last, and later iterations fill the blocks of _V_ from the first to the last. Within an iteration, the blocks are filled in ascending order of index, and _j_ is the index of the block being filled.
- The index _prev_ is one less than _j_, or one less than _q_ when _j_ is 0 in a later iteration.
- The candidate set holds blocks of _V_ in ascending order of index.
    - In the first iteration, it holds the blocks from the first up to the block whose index is two less than _j_, so its size is one less than _j_.
    - In later iterations, it holds every block except the block at index _prev_, so its size is one less than _q_.
- _J1_ is the low 32 bits of the first word of the block of _V_ at index _prev_. _J2_ is not computed. _J1_ selects a position _pos_ in the candidate set as in RFC 9106 Section 3.4.2.
- The position _pos_ refers to the block index _z_, which is _pos_ if _pos_ is less than _prev_, and one more than _pos_ otherwise.
- The block at index _j_ is set to **G** of the block at index _prev_ and the block at index _z_. Later iterations overwrite the block at index _j_, where the RFC XORs the result into its existing value. Both blocks are read before the block at index _j_ is overwritten, so when _z_ equals _j_ in a later iteration, the previous value of the block at index _j_ is used.

## Evaluating a Nonce Value

First, a block _X_ MUST be derived from the input and a nonce of type _N_, which is encoded as the little-endian bytes of that type.

```
prefix = BLAKE3(input)
X = XOF(prefix || nonce || tag, 1024)       // Trustless
X = KXOF(key, prefix || nonce || tag, 1024) // Permissioned
```

Next, _X_ MUST be mixed over _e_ rounds, following the second loop of [scryptROMix](https://www.rfc-editor.org/info/rfc7914/#section-5), with these differences:

- The loop runs for _e_ rounds, independently of the length of _V_.
- Integerify is narrowed to the last 8 octets of _X_, and the length of _V_ is not constrained to a power of two.

In each round, _j_ is the last word of _X_ modulo _q_, _X_ is set to _X_ XOR the block of _V_ at index _j_, and then _X_ is set to **BlockMix** of _X_.

Finally, the digest is **BLAKE3** of _X_, using the regular mode in both modes.

A digest satisfies the difficulty when its first _n_ divided by 8 octets, rounded down, are zero, and the _n_ modulo 8 most significant bits of the next octet are zero.

## Mining and Verifying

Mining searches for a proof over an input. A counter of type _T_ starts at a chosen value, and an increment of type _I_ is added to it each time. Each counter value is converted into a nonce of type _N_, and the search ends when the digest satisfies the difficulty. The nonce that does so is the proof. The runtime is unbounded on high difficulties. As the counter can overflow, its integer type SHOULD be wide enough to reach a satisfying proof, otherwise the counter may wrap around causing an infinite loop.

Verifying a proof over an input evaluates the proof as the nonce. The proof MUST be a value of the same type _N_ that mining returned. The verifying instance MUST have the same mode and configuration as the instance the proof was mined with, and in the Permissioned mode the same shared secret. The verifying instance MUST accept the proof if and only if the digest satisfies the difficulty.

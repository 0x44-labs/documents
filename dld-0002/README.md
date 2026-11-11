# Proof-Bound Claims Protocol

```
DLD:        2
Type:       Specification
Status:     In Progress
Authors:    Haap, Jacob V. B.
Created:    2026-11-10
Updated:    2026-11-10
Requires:   1
```

# Specification

The key words "MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT", "SHOULD", "SHOULD NOT", "RECOMMENDED",  "MAY", and "OPTIONAL" in this document are to be interpreted as described in [RFC 2119](https://datatracker.ietf.org/doc/html/rfc2119).

## Dependencies

- [DLD 1](../dld-0001/README.md) TraverseV Proof-of-Work Algorithm.

## Definitions

- **Application context**: a UTF-8 string.
- **Authentication tag**: a 16-octet value authenticating the fields of a challenge by the issuing verifier's key.
- **BLAKE3(x)**: 32-octet [BLAKE3](https://github.com/BLAKE3-team/BLAKE3-specs/blob/master/blake3.pdf) regular hash of _x_.
- **Canonical claims**: a claims set serialised as JSON in canonical form, as described in [RFC 8785](https://www.rfc-editor.org/info/rfc8785/).
- **Challenge**: a verifier-issued value carrying a challenge identifier, expiration, challenge nonce, **DLD 1** parameters, and an authentication tag.
- **Challenge identifier**: a [ULID](https://github.com/ulid/spec).
- **Challenge nonce**: an octet string of at most 64 octets.
- **Claims set**: a JSON object whose members are the claims, as described in [RFC 7519 Section 4](https://www.rfc-editor.org/info/rfc7519/#section-4).
- **Counter**: a 128-bit unsigned integer, holding a count in the low 64 bits and a Unix time, in seconds, in the high 64 bits.
- **Expiration**: the Unix time, in seconds, after which a challenge is no longer valid.
- **Key**: an octet string of 32 octets, held by a verifier.
- **KXOF(K, x, L)**: first _L_ octets of the BLAKE3 keyed hash extendable output of _x_, with the 32-octet key _K_.
- **Proof**: a response type carrying a challenge, claims set, and counter that is a **DLD 1** proof over the canonical claims.
- **Prover**: the role that responds to a challenge with a proof.
- **Replay store**: a record of used challenge identifiers, provided to a verifier and shareable by multiple verifiers.
- **Verifier**: the role that issues challenges and verifies proofs.

## Parameters

A Proof-Bound Claims Protocol (PBCP) exchange is configured by the parameters of **DLD 1** and by a time to live (TTL). An implementation **MUST** reject values outside their ranges.

| Name | Meaning                            | Range         |
|------|------------------------------------|---------------|
| ttl  | challenge time to live, in seconds | 0 to 2^64 - 1 |

## Context

A PBCP context is created from an application context and **DLD 1** parameters. These parameters are those the challenges its verifiers issue carry and the ceiling for the challenges its provers accept. It creates the verifiers and provers of a PBCP exchange.

A verifier is created from a context, a key, and a TTL. It takes its application context and **DLD 1** parameters from the context.

A prover is created from a context and a challenge. It takes its application context from the context, and its **DLD 1** parameters from the challenge. A prover **SHOULD NOT** be created for a challenge if any of its parameter exceed the ceiling set by the context.

## Proof-of-Work Instance

In both roles, a secret **MUST** be derived from the challenge identifier _id_ and the challenge nonce _nonce_.

```
secret = BLAKE3(id || nonce)
```
_Figure 1: Challenge secret computation._

The proof-of-work instance used by both roles is a **DLD 1** TraverseV instance in the Permissioned mode. To issue, respond to, or verify a challenge, both roles **MUST** create a proof-of-work instance from their application context, the **DLD 1** parameters of the challenge, and the challenge secret as the shared secret.

## Challenge Issuance

A verifier issues a challenge over a challenge nonce _nonce_. The challenge identifier _id_ is a ULID, the expiration *expires_at* is the current Unix time plus the verifier's TTL, and the parameters _m_, _t_, _e_, and _n_ are the **DLD 1** parameters of the verifier.

An authentication tag **MUST** be computed with the key of the verifier _key_ over the fields of the challenge.

```
tag = KXOF(key, id || LE64(expires_at)
        || LE32(m) || LE32(t) || LE32(e) || LE32(n)
        || nonce, 16)
```
_Figure 2: Authentication tag computation._

Since a verifier can recompute the authentication tag of a challenge it issued, it needs no stored state to recognise it in a response.

## Responding to a Challenge

A prover responds to a challenge with a proof over a claims set.

### Counter

The counter is the counter type and the nonce type of **DLD 1**.

An increment is a 64-bit unsigned integer. Adding it to a counter adds it to the count of the counter and leaves its time unchanged. If the sum would exceed 2^64 - 1, the count is instead set to zero and the time is set to the current Unix time in seconds.

As a nonce, a counter is encoded as the 16 little-endian octets of its 128-bit value, so the first 8 octets encode the count and the last 8 octets encode the time.

### Proof Generation

The prover creates a proof-of-work instance from the challenge, and takes the canonical claims of the claims set as the mining input.

Mining follows the process defined in **DLD 1**, with an increment of 1 and a starting counter that has a count of zero and the current Unix time. The mining search returns a counter.

A proof carries the challenge it responds to, the claims set, and the mined counter.

## Verifying a Response

A verifier **MUST** accept a proof if and only if all of the following hold.

1. An authentication tag computed as in Figure 2, from the fields of the proof's challenge and the verifier's key, equals the authentication tag of the challenge. An implementation **SHOULD** perform this check in constant time.
2. Neither the time of the mined counter nor the current Unix time is greater than the expiration of the challenge.
3. The mined counter is a **DLD 1** proof over the canonical claims, for the proof-of-work instance created from the challenge.

## Replay Protection

A challenge is single-use. Without a replay store, a verifier accepts every valid proof over a challenge until its expiration, meaning the same proof can be submitted again and multiple proofs can be generated for the same challenge. A verifier **MAY** use a replay store to enforce single use.

A verifier using a replay store **MUST** accept a proof only if, in addition to the conditions described under "Verifying a Response", it has not been previously recorded and the replay store records the challenge identifier of the proof when first seen. Concurrent submissions of proofs over the same challenge **MUST NOT** result in more than one of them being accepted.

A recorded challenge identifier **MUST** be retained at least until the expiration of the challenge. If the clocks of verifiers sharing a replay store, or of a verifier and its replay store, can differ, the identifier **SHOULD** be retained beyond the expiration by at least that difference.
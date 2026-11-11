# Proof-Bound Claims Protocol

```
DLD:        2
Type:       Specification
Status:     In Progress
Authors:    Haap, Jacob V. B.
Created:    2026-11-10
Updated:    2026-11-11
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
- **Challenge identifier**: a [ULID](https://github.com/ulid/spec). As an input to BLAKE3 or KXOF, it is encoded as a 128-bit big-endian integer.
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

| Name | Meaning                             | Range                   |
|------|-------------------------------------|-------------------------|
| ttl  | challenge time to live, in seconds  | 0 to 2^64 - 1           |
| m    | **DLD 1** parameter _m_             | as defined by **DLD 1** |
| t    | **DLD 1** parameter _t_             | as defined by **DLD 1** |
| e    | **DLD 1** parameter _e_             | as defined by **DLD 1** |
| n    | **DLD 1** parameter _n_             | as defined by **DLD 1** |

## Context

A PBCP context is created from an application context and **DLD 1** parameters. These parameters are those the challenges its verifiers issue carry and the ceiling for the challenges its provers accept. It creates the verifiers and provers of a PBCP exchange.

A verifier is created from a context, a key, and a TTL. It takes its application context and **DLD 1** parameters from the context. A prover is created from a context and a challenge. It takes its application context from the context, and its **DLD 1** parameters from the challenge.

## Proof-of-Work Instance

In both roles, a secret **MUST** be derived from the challenge identifier _id_ and the challenge nonce _nonce_.

```
secret = BLAKE3(id || nonce)
```
_Figure 1: Challenge secret computation._

The proof-of-work instance used by both roles is a **DLD 1** TraverseV instance in the Permissioned mode. To issue, respond to, or verify a challenge, both roles **MUST** create a proof-of-work instance from their application context, the **DLD 1** parameters of the challenge, and the challenge secret as the shared secret.

## Counter Type

The counter is the counter type and the nonce type of **DLD 1**.

An increment is a 64-bit unsigned integer. Adding it to a counter adds it to the count of the counter and leaves its time unchanged. If the sum would exceed 2^64 - 1, the count is instead set to zero and the time is set to the current Unix time in seconds.

As a nonce, a counter is encoded as the 16 little-endian octets of its 128-bit value, so the first 8 octets encode the count and the last 8 octets encode the time.

## Exchange

During a PBCP exchange, a verifier issues a challenge to a prover, and a prover responds with a proof over a claims set.

```
Verifier -> Prover: challenge = (id, expires_at, nonce, m, t, e, n, tag)
Prover -> Verifier: proof = (challenge, claims, counter)
```
_Figure 2: Challenge and proof exchange._

The **DLD 1** parameters _m_, _t_, _e_, and _n_, carried by a challenge on both sides of an exchange are optional and can be omitted. Any parameter that is omitted takes a default value.

| Parameter | Default Value |
|-----------|---------------|
|     m     | 19 * 1024     |
|     t     | 3             |
|     e     | 8             |
|     n     | 20            |

Provers and verifiers **MUST** reject any challenge or proof that contains a field not defined for its type.

### Challenge

A challenge is a value that a verifier issues to a prover, carrying a required challenge identifier, expiration, challenge nonce, and authentication tag.

| Field      | Value                | Presence |
|------------|----------------------|----------|
| id         | challenge identifier | REQUIRED |
| expires_at | expiration           | REQUIRED |
| nonce      | challenge nonce      | REQUIRED |
| m_cost     | _m_                  | OPTIONAL |
| t_cost     | _t_                  | OPTIONAL |
| e_cost     | _e_                  | OPTIONAL |
| n_cost     | _n_                  | OPTIONAL |
| auth_tag   | authentication tag   | REQUIRED |

### Response

A proof is a response that a prover returns to a verifier for a challenge, carrying a required challenge identifier, expiration, authentication tag, claims set, and a mined counter.

| Field      | Value                | Presence |
|------------|----------------------|----------|
| id         | challenge identifier | REQUIRED |
| expires_at | expiration           | REQUIRED |
| claims     | claims set           | REQUIRED |
| challenge  | challenge nonce      | REQUIRED |
| nonce      | mined counter        | REQUIRED |
| m_cost     | _m_                  | OPTIONAL |
| t_cost     | _t_                  | OPTIONAL |
| e_cost     | _e_                  | OPTIONAL |
| n_cost     | _n_                  | OPTIONAL |
| auth_tag   | authentication tag   | REQUIRED |

The fields of a proof other than `claims` and `nonce` are the fields of its challenge.

## Challenge Issuance

A verifier issues a challenge over a challenge nonce _nonce_, and **MUST NOT** issue a challenge over a challenge nonce longer than 64 octets. The challenge identifier _id_ is a ULID, the expiration *expires_at* is the current Unix time plus the verifier's TTL, and the parameters _m_, _t_, _e_, and _n_ are the **DLD 1** parameters of the verifier.

An authentication tag **MUST** be computed with the key of the verifier _key_ over the fields of the challenge.

```
tag = KXOF(key, id || LE64(expires_at)
        || LE32(m) || LE32(t) || LE32(e) || LE32(n)
        || nonce, 16)
```
_Figure 3: Authentication tag computation._

Since a verifier can recompute the authentication tag of a challenge it issued, it needs no stored state to recognise it in a response.

## Responding to a Challenge

A prover responds to a challenge with a proof over a claims set.

### Prover Checks

A prover **MUST NOT** be created for a challenge whose expiration has passed, and **SHOULD NOT** be created for a challenge if any of its parameters exceed the ceiling set by the context.

### Proof Generation

The prover creates a proof-of-work instance from the challenge, and takes the canonical claims of the claims set as the mining input.

Mining follows the process defined in **DLD 1**, with an increment of 1 and a starting counter that has a count of zero and the current Unix time. The mining search returns a counter.

A proof carries the challenge it responds to, the claims set, and the mined counter.

## Verifying a Response

A verifier **MUST** accept a proof if and only if all of the following hold.

1. An authentication tag computed as in Figure 3, from the fields of the proof's challenge and the verifier's key, equals the authentication tag of the challenge. An implementation **SHOULD** perform this check in constant time.
2. Neither the time of the mined counter nor the current Unix time is greater than the expiration of the challenge.
3. The mined counter is a **DLD 1** proof over the canonical claims, for the proof-of-work instance created from the challenge.
4. If the verifier uses a replay store, the requirements of "Replay Protection" are satisfied.

### Replay Protection

A challenge is single-use. Without a replay store, a verifier accepts every valid proof over a challenge until its expiration, meaning the same proof can be submitted again and multiple proofs can be generated for the same challenge. A verifier **MAY** use a replay store to enforce single use.

A verifier using a replay store **MUST** accept a proof only if, in addition to conditions 1 through 3 under "Verifying a Response", its challenge identifier has not been previously recorded in the replay store, which records it when first seen. Concurrent submissions of proofs over the same challenge **MUST NOT** result in more than one of them being accepted.

A recorded challenge identifier **MUST** be retained at least until the expiration of the challenge. If the clocks of verifiers sharing a replay store, or of a verifier and its replay store, can differ, the identifier **SHOULD** be retained beyond the expiration by at least that difference.

## Encodings

A challenge is serialised as bytes or as a URI, and a proof is serialised as bytes.

### Byte Serialisation

A challenge or proof is serialised as a CBOR map, as described in [RFC 8949](https://www.rfc-editor.org/info/rfc8949/), with a UTF-8 string key for each field of its type. Field values are encoded as follows:

- **Challenge identifier**: a UTF-8 string, in its canonical ULID representation.
- **Expiration** and **parameters**: unsigned integers.
- **Challenge nonce** and **authentication tag**: octet strings.
- **Claims set**: an octet string of its canonical claims.
- **Mined counter**: an octet string of its encoding as a nonce.

### URI Serialisation

A challenge is serialised as a URI with the scheme `pbcp` and an authority that names where a proof should be returned.

```
pbcp://<authority>/?id=<id>&expires_at=<time>&nonce=<nonce>&auth_tag=<tag>
        [&m_cost=<m>][&t_cost=<t>][&e_cost=<e>][&n_cost=<n>]
```
_Figure 4: PBCP Challenge URI._

The authority is as described in [RFC 3986 Section 3.2](https://www.rfc-editor.org/info/rfc3986/#section-3.2). Query parameter values are encoded as follows:

- **Challenge identifier**: its canonical ULID representation.
- **Expiration** and **parameters**: decimal integers.
- **Challenge nonce** and **authentication tag**: base64url without padding, as described in [RFC 4648 Section 5](https://www.rfc-editor.org/info/rfc4648/#section-5).
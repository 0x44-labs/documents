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

## Definitions

- **Application context**: a UTF-8 string.
- **Authentication tag**: a 16-octet value authenticating the fields of a challenge by the issuing verifier's key.
- **BLAKE3(x)**: 32-octet [BLAKE3](https://github.com/BLAKE3-team/BLAKE3-specs/blob/master/blake3.pdf) regular hash of _x_.
- **Canonical claims**: a claims set serialised as JSON in canonical form, as described in [RFC 8785](https://www.rfc-editor.org/info/rfc8785/).
- **Challenge**: a verifier-issued value carrying a challenge identifier, expiration, challenge nonce, parameters, and an authentication tag.
- **Challenge identifier**: a [ULID](https://github.com/ulid/spec), read as its 16-octet binary form.
- **Challenge nonce**: an octet string of at most 64 octets.
- **Claims set**: a JSON object whose members are the claims, as described in [RFC 7519 Section 4](https://www.rfc-editor.org/info/rfc7519/#section-4).
- **Counter**: a 128-bit unsigned integer, holding a count in the low 64 bits and a Unix time in the high 64 bits.
- **Expiration**: the Unix time after which a challenge is no longer valid.
- **Key**: a 32-octet octet string, held by a verifier.
- **KXOF(K, x, L)**: first _L_ octets of the BLAKE3 keyed hash extendable output of _x_, with the 32-octet key _K_.
- **Proof**: a challenge, a claims set, and a counter that is a proof over the canonical claims.
- **Prover**: the role that responds to a challenge with a proof.
- **Replay store**: a record of used challenge identifiers.
- **Verifier**: the role that issues challenges and verifies proofs.

## Dependencies

- [DLD 1](../dld-0001/README.md) TraverseV Proof-of-Work Algorithm.

## Parameters

PBCP is configured by the parameters of DLD 1 and by a time to live (TTL). An implementation **MUST** reject values outside their ranges.

| Name | Meaning                            | Range         |
|------|------------------------------------|---------------|
| ttl  | challenge time to live, in seconds | 0 to 2^64 - 1 |

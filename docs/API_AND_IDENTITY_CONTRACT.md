# API and identity contract for 0.1

This document fixes the intended public boundary for the first registry
release. It distinguishes mathematical equality, representation identity, and
numerical thermograph output. Those quantities answer different questions and
must not be substituted for one another.

## Mathematical domain

The bounded exact APIs accept explicitly constructed finite, loop-free,
two-player partizan games under normal play. Each side's options have set
semantics. Ruleset legality, reachability, terminal detection, and completeness
remain the caller's responsibility.

The exact contract excludes draws, repetition, loopy or transfinite games,
misere play, chance, hidden information, and incomplete move graphs.

## Exact operations

The following operations are mathematical within the stated domain and only
return a verdict after all configured resource and semantic audits pass:

- `compare_short_game_bounded`;
- `equal_short_game_bounded`;
- `semantic_canonical_form_bounded`; and
- `semantic_target_catalogue_birthday3_bounded`.

An `Err(ShortGameError)` carries no comparison verdict or semantic identity.
`ShortGameProfile::order7_v1` and `ShortGameProfile::digraph8_v1`, including
their profile IDs and numeric limits, are immutable named profiles. The
`max_certificate_bytes` field is reserved metadata; version 0.1 exposes no
certificate serializer and does not enforce that field.

`DyadicRational` normalizes values and implements exact equality and ordering
for every representable numerator and denominator power. Checked arithmetic
returns `None` when the normalized result cannot fit an `i32` numerator.

## Approximate operations

`CGTValue::approximate_thermograph` and `ApproximateThermograph` use `f32` for
breakpoints, slopes, means, and temperatures. Their results support numerical
exploration and visualization. They do not establish CGT equality or semantic
identity.

`CGTValue::exact_thermograph` is a deprecated source-compatibility wrapper. Its
return values are the same floating-point approximation. The shorter
`thermograph`, `temperature`, `mean_value`, `left_scaffold`, and
`right_scaffold` methods are compatibility accessors to that approximate
calculation.

## Identity layers

### Legacy structural identity

The following outputs are frozen for the 0.1 line:

- `CGTValue::canonical_serialization`;
- `CGTValue::canonical_bytes`;
- `CGTValue::stable_canonical_hash`;
- `CGTValue::stable_canonical_digest`;
- `CGTValue::canonical_payload_v1_bytes`; and
- `CGTValue::digest_v1_sha256`.

Structural canonicalization normalizes numeric atoms and treats each option
list as a sorted set. It does not reduce arbitrary equal games to one value.
The FNV digest remains a compatibility field and is not collision-resistant.

### Literal identity

`expand_short_game_bounded` expands all atoms into braces and reports both the
legacy unprefixed literal SHA-256 and the domain-separated v1 literal SHA-256.
The prefix `partizan.explicit_short_game.v1\n` and brace encoding are frozen.

### Semantic identity

`semantic_canonical_form_bounded` reduces an explicit short game and computes

```text
sha256("partizan.semantic_canonical_game.v1\n" + canonical_serialization)
```

The prefix, brace encoding, set semantics, reduction order, and catalogue sort
order are frozen. Equivalent representations may have different structural
digests while sharing a semantic value ID.

The file `conformance/day2-semantic-ids-v1.txt` is an immutable cross-runtime
fixture for the complete 256-game literal domain through birthday two.

## Compatibility policy

Patch releases in the 0.1 line may add tests, documentation, checked APIs, or
new explicitly versioned formats. They will not change existing payload bytes,
digests, profile IDs, semantic IDs, catalogue ordering, or successful exact
verdicts. A required format change receives a new domain separator and a
migration document. A mathematical correction affecting an existing verdict
is documented prominently with a minimal counterexample and independent
validation.

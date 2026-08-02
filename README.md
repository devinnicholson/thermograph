# Thermograph

[![CI](https://github.com/devinnicholson/thermograph/actions/workflows/ci.yml/badge.svg)](https://github.com/devinnicholson/thermograph/actions/workflows/ci.yml)

Thermograph is a Rust research library for finite, loop-free, normal-play
partizan games. It provides:

- bounded exact comparison and mathematical equality;
- deterministic semantic canonical forms and domain-separated value IDs;
- the complete 1,474-value catalogue born by birthday three;
- exact normalized dyadic arithmetic;
- stable legacy structural identities; and
- approximate floating-point thermography.

Rust 1.85 or newer is required. The crate has no runtime dependencies.

## Status and role

| Item | Current state |
| --- | --- |
| Crate | `thermograph` 0.1.0 research candidate |
| Exact domain | Explicit finite, loop-free, two-player normal-play games |
| Minimum Rust | 1.85 |
| Runtime dependencies | None |
| License | MIT OR Apache-2.0 |
| Registry release | Pending |

Thermograph is the exact finite-game algebra layer used by
[Partizan](https://github.com/devinnicholson/partizan). Partizan decides which
candidate to examine; a ruleset adapter constructs its complete game;
Thermograph compares and canonicalizes that explicit game. Optional chess-side
structure and search live in [Bitmesh](https://github.com/devinnicholson/bitmesh)
and [Astralbase](https://github.com/devinnicholson/astralbase).

These boundaries are part of the API contract. Thermograph does not decide
whether a chess, Domineering, or other ruleset position is legal, complete, or
reachable. An integration must independently validate its move generator and
terminal semantics before treating a constructed game as ruleset evidence.

## Quick start

```console
git clone https://github.com/devinnicholson/thermograph.git
cd thermograph
cargo test --locked
cargo run --locked --example semantic
cargo run --locked --example switch
```

## Identity contracts

Thermograph keeps representation and mathematical identity separate.

| Identity | API | Meaning |
| --- | --- | --- |
| Legacy structural | `CGTValue::canonical_serialization` | Normalized numeric atoms and recursively sorted, deduplicated option syntax |
| Legacy structural FNV | `CGTValue::stable_canonical_digest` | Compatibility digest of the legacy structural serialization |
| Structural SHA-256 | `CGTValue::digest_v1_sha256` | Domain-separated SHA-256 of the legacy structural payload |
| Explicit literal | `expand_short_game_bounded` | Atom-expanded brace tree with set-valued options |
| Semantic value | `semantic_canonical_form_bounded` | Reduced normal-play canonical form and domain-separated SHA-256 `value_id` |

Equivalent games may retain different legacy structural digests. For example,
the dyadic atom `1/2` and the explicit tree `{0 | 1}` share one semantic
`value_id` after bounded canonicalization.

`exact_value_payload` retains its compatibility contract. Its exact numeric
field is available for the `Number` class. Other classes carry exact payload
bytes and structural identity.

## Bounded exact short games

The `short_game` module expands integers, dyadics, `Star`, `Up`, and `Down`
into explicit finite trees. It then interns collision-safe node keys and
memoizes Conway's recursive order relation.

The four comparison results are:

- `Less`
- `Equal`
- `Greater`
- `Fuzzy`

Every bounded failure returns `ShortGameError`. Resource failures identify the
resource, configured limit, and observed count. No comparison verdict or
semantic identifier accompanies a failed request.

### Named profiles

| Resource | `order7.v1` | `digraph8.v1` |
| --- | ---: | ---: |
| Literal and canonical birthday | 7 | 8 |
| Source nodes per root | 128 | 256 |
| Options per side | 7 | 8 |
| Combined option references | 1,792 | 4,096 |
| Combined intermediate nodes | 4,096 | 8,192 |
| Comparison pairs | 262,144 | 1,000,000 |
| Reduction rewrites | 4,096 | 8,192 |
| Serialization bytes per root | 16 MiB | 32 MiB |
| Reserved certificate bytes | 64 MiB | 128 MiB |

The stable identifiers are:

```text
partizan.bounded_short_game.order7.v1
partizan.bounded_short_game.digraph8.v1
```

`ShortGameProfile::default()` selects `order7.v1`. Callers may copy a profile
and lower any bound for a stricter operation.

### Semantic quickstart

Run the checked example:

```text
cargo run --locked --example semantic
```

The example verifies the Elkies form `{0, * | 1}` against the dyadic value
`1/2`:

```rust
use thermograph::CGTValue;
use thermograph::short_game::{
    ShortGameProfile, semantic_canonical_form_bounded,
};

let elkies = CGTValue::GameTree {
    left: vec![CGTValue::Integer(0), CGTValue::Star],
    right: vec![CGTValue::Integer(1)],
};
let profile = ShortGameProfile::order7_v1();
let left = semantic_canonical_form_bounded(&elkies, &profile)?;
let right =
    semantic_canonical_form_bounded(&CGTValue::Dyadic(1, 1), &profile)?;
assert_eq!(left.value_id, right.value_id);
# Ok::<(), thermograph::short_game::ShortGameError>(())
```

Canonical reduction recursively reduces followers, coalesces options, removes
dominated options, bypasses reversible options, and repeats to a deterministic
fixed point. Each completed call audits irreducibility, exact equality with the
input, and byte-idempotence.

## Birthday-three target catalogue

`semantic_target_catalogue_birthday3_bounded` constructs the finite catalogue
from the mathematical order:

1. generate the cumulative birthday-zero, birthday-one, and birthday-two
   values;
2. compute the order relation among the 22 birthday-two values;
3. enumerate its 98 antichains;
4. canonicalize all 9,604 ordered Left/Right antichain pairs; and
5. sort the resulting rows by semantic value ID and serialization.

The checked counts are:

| Birthday | Exact count | Cumulative count |
| ---: | ---: | ---: |
| 0 | 1 | 1 |
| 1 | 3 | 4 |
| 2 | 18 | 22 |
| 3 | 1,452 | 1,474 |

Birthday four has a vastly larger known search space and lies outside this
catalogue API.

## Approximate thermography

`CGTValue::approximate_thermograph` returns `f32` breakpoints, scaffolds, mean,
and temperature. Tests state explicit absolute tolerances. These fields serve
numerical exploration and visualization; semantic identities come from the
bounded exact APIs.

For number atoms, temperature `-1.0` is an implementation convention.
Standard infinitesimals in the fixture corpus report mean and temperature
`0.0` while retaining distinct game identities.

Run the checked switch example:

```text
cargo run --locked --example switch
```

Expected output:

```text
approximate temperature=1.000000 mean=0.000000 tolerance=0.000001
```

The deprecated `exact_thermograph` method remains source-compatible and
returns the same floating-point result as `approximate_thermograph`.

## Supported scope

The exact APIs cover explicitly constructed finite, loop-free, two-player
partizan games under normal play. Options have set semantics.

Excluded domains:

- draws and repetition;
- loopy and transfinite games;
- misère play;
- chance and hidden information; and
- ruleset-specific legality.

The legacy `CGTValue::ge`, `le`, and `simplify` methods retain their original
unbounded behavior. New research code should select a `ShortGameProfile` and
use the bounded APIs.

## Validation

Run all release checks:

```text
cargo fmt --check
cargo clippy --locked --all-targets --all-features -- -D warnings
cargo test --locked
cargo rustdoc --locked --all-features --lib -- -D warnings -D missing-docs
cargo package --locked
```

The test suite includes:

- byte-stability fixtures for every legacy digest surface;
- atom/tree equivalence and fuzzy controls;
- domination, reversibility, negation, and disjunctive sums;
- the Elkies `{0, * | 1} = 1/2` reduction;
- canonical equality, irreducibility, and idempotence audits;
- all 256 birthday-two literal games, yielding 22 semantic values; and
- the complete birthday-three catalogue, yielding 1,474 semantic values.

Reference expectations follow:

- John H. Conway, *On Numbers and Games*, second edition, A K Peters, 2001.
- Elwyn R. Berlekamp, John H. Conway, and Richard K. Guy, *Winning Ways for
  Your Mathematical Plays*, second edition, volume 1, A K Peters, 2001.
- Aaron N. Siegel, *Combinatorial Game Theory*, American Mathematical Society,
  2013.

## Citation

Citation metadata is available in [`CITATION.cff`](CITATION.cff). A release
archive or DOI should be used when one is available.

## License

Thermograph is available under either:

- [MIT](LICENSE-MIT)
- [Apache License 2.0](LICENSE-APACHE)

# Thermograph

[![CI](https://github.com/devinnicholson/thermograph/actions/workflows/ci.yml/badge.svg)](https://github.com/devinnicholson/thermograph/actions/workflows/ci.yml)

Thermograph is an experimental Rust library for finite, normal-play,
partizan combinatorial games. It provides exact dyadic-number arithmetic,
structural game-tree labels, a small set of named values, and an approximate
floating-point thermograph implementation.

The crate is useful for reproducible experiments with small game trees. It is
not yet a general-purpose CGT proof system or a reference implementation of all
thermography.

## Contract and terminology

The following distinctions are part of the public contract:

- **Exact number** means a normalized dyadic rational `n / 2^k` represented by
  `DyadicRational`. Integer/dyadic addition, subtraction, and negation use exact
  integer arithmetic when the normalized numerator fits in `i32`.
- **Structural identity** means the versioned serialization produced by
  `canonical_serialization`: numeric atoms are normalized and each option list
  is sorted and deduplicated recursively. It identifies the represented tree
  modulo option order and duplicate options.
- **Canonical** in API names refers only to that structural normalization. It
  does **not** prove equality in the normal-play game group and does not promise
  that dominated or reversible options have been removed. Two equivalent games
  can therefore have different structural digests.
- **Approximate thermography** means the `f32` piecewise-linear calculation
  returned by `approximate_thermograph`. Breakpoints, mean, temperature, and
  scaffold evaluations are numerical approximations. Tests use an explicit
  absolute tolerance; these values must not be used as exact identities.
- `exact_value_payload` is exact only about the serialized payload and, for the
  `Number` class, its dyadic numeric value. `Star`, `Up`, `Down`, `Switch`, and
  `GameTree` payloads do not contain exact numeric values.

The historical method `exact_thermograph` is retained for source compatibility
but deprecated because its name overstates an `f32` result. New code should use
`approximate_thermograph`.

## Supported domain and non-claims

This release models explicitly constructed, finite, loop-free, normal-play
game trees. It does not model draws, repetition, loopy games, misere play,
infinite games, or game-specific legality. `simplify` is a useful recursive
reduction for the supported representation, but it is not certified as a
complete canonical-form algorithm for arbitrary CGT inputs. The comparison and
thermograph routines have no hard resource bound and may expand rapidly on
deep or highly branching trees.

For number atoms this crate uses temperature `-1.0` as an implementation
convention. Standard infinitesimals in the fixture corpus have mean and
temperature reported as `0.0`; their game identity is not thereby reduced to
the number zero.

## Executable quickstart

Run the checked switch example from a clean checkout:

```text
cargo run --locked --example switch
```

Expected output (to six decimal places):

```text
approximate temperature=1.000000 mean=0.000000 tolerance=0.000001
```

The example constructs the switch `{ 1 | -1 }`:

```rust
use thermograph::CGTValue;

// The switch { 1 | -1 } has hand-computable mean 0 and temperature 1.
let game = CGTValue::GameTree {
    left: vec![CGTValue::Integer(1)],
    right: vec![CGTValue::Integer(-1)],
};

let approximation = game.approximate_thermograph();
assert!((approximation.temperature - 1.0).abs() <= 1e-6);
assert!((approximation.mean - 0.0).abs() <= 1e-6);
```

These bounded checks validate this implementation's approximate `f32` result
for one hand-computable fixture. They are not a claim of exact thermography or
a proof that arbitrary game trees have been reduced to canonical form.

Run all checks with a Rust 1.85 or newer toolchain:

```text
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --locked
cargo rustdoc --lib -- -D missing-docs
```

## Stable value-digest contract

For `BMCOMPOSE` component labels, copy `value_digest` from
`CGTValue::exact_value_payload().digest`. That digest is the stable FNV-1a
digest of the legacy canonical serialization. For a collision-resistant,
domain-separated digest, prefer `digest_v1_sha256()`.

Do not derive identity from `temperature()`, `mean_value()`, `thermograph()`,
scaffolds, or display text. The byte-stability fixtures cover exact numbers,
simple switches, and structural game trees. Approximate composition is not
implemented; downstream payloads should explicitly mark it unsupported.

## Validation and references

`tests/reference_corpus.rs` contains independently hand-verifiable fixtures for
numbers, switches, star/up/down, sums, negation, dominated options, reversible
options, mean, and temperature. The expected values follow the definitions and
standard examples in:

- John H. Conway, *On Numbers and Games*, second edition, A K Peters, 2001.
- Elwyn R. Berlekamp, John H. Conway, and Richard K. Guy, *Winning Ways for
  Your Mathematical Plays*, second edition, volume 1, A K Peters, 2001.
- Aaron N. Siegel, *Combinatorial Game Theory*, American Mathematical Society,
  2013.

These fixtures are a small regression and validation corpus, not an exhaustive
cross-validation against a separate CGT implementation.

## Research context

The crate was created to investigate whether structural game values and
temperature-like features can be useful in chess-oriented learning systems.
That is a research hypothesis, not an empirical performance claim. See
`CHANGELOG.md` for API history and `CONTRIBUTING.md` for validation expectations.

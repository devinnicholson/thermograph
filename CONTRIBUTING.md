# Contributing

Thermograph is a research prototype. Changes should make its mathematical and
numerical contracts easier to inspect, reproduce, and falsify.

## Before contributing

Thermograph is dual-licensed MIT OR Apache-2.0 (see `LICENSE-MIT` and
`LICENSE-APACHE`). Contributor terms such as a CLA or DCO remain undecided.
Maintainers should settle those terms before soliciting or merging external
contributions. Contributors must have the right to submit every included code,
data, and documentation asset.

## Development checks

Use Rust 1.85 or newer and run:

```text
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --locked
cargo rustdoc --lib -- -D missing-docs
```

All four commands must pass before review. Keep public APIs documented and
preserve stable digest fixtures unless the payload version is intentionally
incremented.

## Mathematical changes

A change to comparison, simplification, arithmetic, scaffolds, temperature, or
mean must include:

1. the supported game domain and exact claim being changed;
2. a hand derivation or an expectation from an independent reference;
3. positive and negative fixtures with explicit floating-point tolerances;
4. a note describing whether structural payload bytes or digests change; and
5. complexity or resource implications for recursive inputs.

Expected values must come from a hand derivation, frozen independent
implementation, or published dataset. Record the oracle name, version, input,
command, raw output, and artifact hash.

## Compatibility

The `canonical_serialization`, `canonical_bytes`,
`stable_canonical_digest`, `canonical_payload_v1_bytes`, and
`digest_v1_sha256` outputs are compatibility surfaces. A change requires a new
payload version, migration notes, and fixtures for both the old and new
formats. Deprecate misleading public APIs before removal and retain them until
a SemVer-compatible release permits removal.

The brace literal prefixes, semantic canonical prefix, named profile
identifiers, and birthday-three catalogue ordering are also compatibility
surfaces. Changes require versioned identifiers and old/new conformance
fixtures.

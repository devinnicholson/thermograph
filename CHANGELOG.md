# Changelog

All notable changes to this project are documented here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and releases are
intended to follow [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### Added

- Precise public contracts for exact dyadic values, structural identity, and
  approximate floating-point thermography.
- Bounded constructor expansion, exact four-way comparison, and semantic
  equality for finite normal-play short games.
- Named `order7.v1` and `digraph8.v1` resource profiles with typed failures.
- Deterministic semantic canonical forms with domain-separated SHA-256 value
  identifiers.
- Complete semantic target catalogue through birthday three: 1,474 values from
  98 antichains and 9,604 ordered option-set pairs.
- Checked semantic example for the Elkies form `{0, * | 1} = 1/2`.
- `ApproximateThermograph` and `CGTValue::approximate_thermograph` as the
  explicitly approximate thermograph API.
- Strict public API documentation and a hand-verifiable reference corpus.
- Cross-platform CI, citation metadata, and contribution guidance.
- An immutable 22-ID birthday-two cross-runtime conformance fixture generated
  from all 256 literal games.
- Exact total ordering for normalized dyadic rationals, including denominator
  powers beyond the representable `f32` range.
- Security, support, DCO, release, and supply-chain policies for the first
  registry release.

### Changed

- Package metadata now states the supported scope and minimum Rust version.
- The README assigns separate names and digests to structural, literal, and
  semantic identities.
- The legacy structural serialization and digest bytes retain their compatibility
  contract.
- Removed the undocumented exploratory `test_bin` executable from the package
  surface; checked examples now cover both thermography and semantic identity.
- Licensed the dependency-free crate under MIT OR Apache-2.0.
- Restricted the registry archive to reviewed source, tests, examples,
  conformance fixtures, licenses, citation metadata, and release notes.

### Fixed

- `CGTValue::value_class()` and `exact_value_payload().value_class` now compare
  dyadic switch options exactly at every representable denominator power.
  Previously `{1/2^200 | 1/2^201}` was classified as `GameTree` after both
  options underflowed to the same `f32`; it is now correctly classified as
  `Switch`. Structural payloads, digests, and semantic value IDs are unchanged.

### Deprecated

- `CGTValue::exact_thermograph`. It remains source-compatible but returns the
  same `f32` approximation as `approximate_thermograph`.

## 0.1.0 - Unreleased

- Initial research prototype.

[Keep a Changelog]: https://keepachangelog.com/en/1.1.0/
[Semantic Versioning]: https://semver.org/spec/v2.0.0.html

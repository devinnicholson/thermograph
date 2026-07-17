# Changelog

All notable changes to this project are documented here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and releases are
intended to follow [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### Added

- Precise public contracts for exact dyadic values, structural identity, and
  approximate floating-point thermography.
- `ApproximateThermograph` and `CGTValue::approximate_thermograph` as the
  explicitly approximate thermograph API.
- Strict public API documentation and a hand-verifiable reference corpus.
- Cross-platform CI, citation metadata, and contribution guidance.

### Changed

- Package metadata now states the supported scope and minimum Rust version.
- The README no longer presents structural normalization as full CGT canonical
  equivalence or thermography as exact for arbitrary game trees.

### Deprecated

- `CGTValue::exact_thermograph`. It remains source-compatible but returns the
  same `f32` approximation as `approximate_thermograph`.

## 0.1.0 - Unreleased

- Initial research prototype.

[Keep a Changelog]: https://keepachangelog.com/en/1.1.0/
[Semantic Versioning]: https://semver.org/spec/v2.0.0.html

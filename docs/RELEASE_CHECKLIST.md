# Release checklist

Every release is built from a clean commit. Registry publication is a manual,
irreversible step and occurs only after the package archive has been inspected
and tested as a downstream dependency.

## Contract review

- [ ] Confirm the supported domain in `docs/API_AND_IDENTITY_CONTRACT.md` still
      matches the implementation.
- [ ] Review every public API change and update `CHANGELOG.md`.
- [ ] Confirm versioned payload prefixes, named profile IDs, golden digests,
      catalogue ordering, and the day-two conformance fixture are unchanged or
      intentionally versioned.
- [ ] Require an independent derivation or oracle for mathematical changes.
- [ ] Confirm `Cargo.toml`, `CITATION.cff`, and the changelog use the release
      version and date.

## Local release gate

Run on Rust 1.85 and current stable:

```text
cargo fmt --check
cargo clippy --locked --all-targets --all-features -- -D warnings
cargo test --locked
cargo rustdoc --locked --all-features --lib -- -D warnings -D missing-docs
cargo deny check
cargo package --locked
cargo package --locked --list
```

- [ ] Inspect the complete `cargo package --list` output.
- [ ] Extract the generated `.crate` archive and run its tests.
- [ ] Create a clean temporary consumer crate using only the packaged archive;
      compile the README semantic example there.
- [ ] Confirm the normal dependency graph contains only `thermograph`.
- [ ] Confirm both license files and citation metadata are present.
- [ ] Confirm docs.rs can build with the declared minimum Rust version.

## Repository gate

- [ ] All required checks pass on Linux, macOS, and Windows.
- [ ] Dependabot and the scheduled `cargo-deny` scan are healthy.
- [ ] No security report is awaiting triage.
- [ ] The release commit is signed or otherwise attestable.
- [ ] The release tag points to the reviewed commit and is protected against
      movement.

## Publication

Run `cargo publish --locked --dry-run` and inspect its output. Publication itself
requires a maintainer's explicit confirmation; automation in this repository
does not publish a crate.

After publication:

- [ ] Install the exact registry version in a clean consumer project.
- [ ] Run the semantic and switch examples from that project.
- [ ] Verify the docs.rs build.
- [ ] Create release notes containing checksums and known limitations.
- [ ] Archive the release and update `CITATION.cff` with an immutable release
      URL or DOI when available.

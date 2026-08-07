## Scope

Describe the change and the public mathematical, identity, numerical, or
packaging contract it touches.

## Evidence

- [ ] New expected values come from a hand derivation, independent oracle, or
      cited published reference.
- [ ] Positive, negative, and resource-bound cases are covered.
- [ ] Floating-point assertions state an absolute tolerance.
- [ ] Existing payload bytes, digests, profile IDs, semantic IDs, and catalogue
      ordering are unchanged, or a new explicit version and migration are included.

## Verification

- [ ] `cargo fmt --check`
- [ ] `cargo clippy --locked --all-targets --all-features -- -D warnings`
- [ ] `cargo test --locked`
- [ ] strict `cargo rustdoc`
- [ ] `cargo package --locked`
- [ ] commits contain DCO `Signed-off-by` lines

List the exact toolchains, commands, and any independent artifacts used.

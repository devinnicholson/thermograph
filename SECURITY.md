# Security policy

## Reporting

Report vulnerabilities through GitHub's private security-advisory interface for
this repository. Do not include exploit details, unpublished identity
collisions, or denial-of-service inputs in a public issue. If private reporting
is unavailable, open a minimal issue requesting a private maintainer contact
without disclosing the vulnerability.

Reports should include the affected commit or crate version, platform and Rust
version, the smallest reproducible input, expected and observed behavior, and
whether the issue crosses a named resource profile.

## Relevant security properties

Security reports include conventional Rust and supply-chain vulnerabilities as
well as:

- a bounded API exceeding a declared resource limit;
- a panic or uncontrolled allocation reachable through a bounded API;
- a collision or ambiguity in a versioned identity encoding;
- a comparison verdict or semantic ID returned after a failed audit;
- malformed input being accepted as evidence for a ruleset Thermograph does not
  validate; and
- a discrepancy between documented exact behavior and the implementation.

Floating-point approximation error within the documented thermography boundary
is a numerical-correctness issue unless it creates a separate security impact.

## Supported versions

Until the first registry release, only the current default branch receives
security fixes. After publication, the current 0.1 patch line is supported.
Older research commits and unpublished archives receive fixes only when the
maintainer states otherwise.

Maintainers aim to acknowledge a complete report within seven days. A fix or
coordinated disclosure date depends on severity and the need for independent
mathematical validation.

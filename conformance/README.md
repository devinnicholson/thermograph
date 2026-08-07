# Cross-runtime conformance fixtures

Files in this directory are versioned interoperability contracts. They are
small enough to ship with the crate and are regenerated from exhaustive finite
domains rather than sampled examples.

`day2-semantic-ids-v1.txt` contains the sorted set of the 22 semantic value IDs
obtained by canonicalizing all 256 literal games whose options come from the
four cumulative birthday-one values. The IDs use the
`partizan.semantic_canonical_game.v1` domain separator.

`tests/day2_conformance.rs` independently regenerates the file from public
Thermograph APIs. Other implementations can generate the same 256-game domain
and compare the resulting sorted ID set byte for byte. Changes require a new
fixture filename and identity version; the v1 file remains immutable.

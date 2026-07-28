//! Checked semantic-canonicalization example for `{0, * | 1} = 1/2`.

use thermograph::CGTValue;
use thermograph::short_game::{ShortGameProfile, semantic_canonical_form_bounded};

fn main() {
    let elkies = CGTValue::GameTree {
        left: vec![CGTValue::Integer(0), CGTValue::Star],
        right: vec![CGTValue::Integer(1)],
    };
    let profile = ShortGameProfile::order7_v1();
    let left = semantic_canonical_form_bounded(&elkies, &profile)
        .expect("bounded Elkies form should canonicalize");
    let right = semantic_canonical_form_bounded(&CGTValue::Dyadic(1, 1), &profile)
        .expect("bounded dyadic half should canonicalize");

    assert_eq!(left.value_id, right.value_id);
    println!(
        "equal=true canonical={} value_id={}",
        left.canonical_serialization, left.value_id
    );
}

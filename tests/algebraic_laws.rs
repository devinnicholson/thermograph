//! Deterministic finite-domain checks of core normal-play algebraic laws.

use thermograph::short_game::{
    ShortGameComparison, ShortGameProfile, compare_short_game_bounded, equal_short_game_bounded,
    semantic_canonical_form_bounded,
};
use thermograph::{CGTValue, DyadicRational, ExactValueClass};

fn small_values() -> Vec<CGTValue> {
    vec![
        CGTValue::Integer(-1),
        CGTValue::Integer(0),
        CGTValue::Dyadic(1, 1),
        CGTValue::Integer(1),
        CGTValue::Star,
        CGTValue::Up,
        CGTValue::Down,
        CGTValue::GameTree {
            left: vec![CGTValue::Integer(1)],
            right: vec![CGTValue::Integer(-1)],
        },
    ]
}

fn reversed(comparison: ShortGameComparison) -> ShortGameComparison {
    match comparison {
        ShortGameComparison::Less => ShortGameComparison::Greater,
        ShortGameComparison::Equal => ShortGameComparison::Equal,
        ShortGameComparison::Greater => ShortGameComparison::Less,
        ShortGameComparison::Fuzzy => ShortGameComparison::Fuzzy,
    }
}

#[test]
fn comparison_is_reflexive_symmetric_and_reversed_by_negation() {
    let profile = ShortGameProfile::order7_v1();
    let values = small_values();

    for left in &values {
        assert_eq!(
            compare_short_game_bounded(left, left, &profile).unwrap(),
            ShortGameComparison::Equal
        );
        for right in &values {
            let forward = compare_short_game_bounded(left, right, &profile).unwrap();
            let backward = compare_short_game_bounded(right, left, &profile).unwrap();
            let negated =
                compare_short_game_bounded(&left.negate(), &right.negate(), &profile).unwrap();

            assert_eq!(backward, reversed(forward));
            assert_eq!(negated, reversed(forward));
        }
    }
}

#[test]
fn disjunctive_sum_is_commutative_and_every_value_cancels_with_its_inverse() {
    let profile = ShortGameProfile::order7_v1();
    let zero = CGTValue::Integer(0);
    let values = small_values();

    for left in &values {
        assert!(equal_short_game_bounded(&left.add(&left.negate()), &zero, &profile).unwrap());
        for right in &values {
            assert!(
                equal_short_game_bounded(&left.add(right), &right.add(left), &profile).unwrap(),
                "disjunctive sum lost commutativity for {left:?} and {right:?}"
            );
        }
    }
}

#[test]
fn canonicalization_is_invariant_under_option_order_and_multiplicity() {
    let profile = ShortGameProfile::order7_v1();
    let variants = [
        CGTValue::GameTree {
            left: vec![CGTValue::Integer(0), CGTValue::Star],
            right: vec![CGTValue::Integer(1)],
        },
        CGTValue::GameTree {
            left: vec![CGTValue::Star, CGTValue::Integer(0)],
            right: vec![CGTValue::Integer(1), CGTValue::Integer(1)],
        },
        CGTValue::GameTree {
            left: vec![
                CGTValue::Star,
                CGTValue::Integer(0),
                CGTValue::Star,
                CGTValue::Integer(0),
            ],
            right: vec![CGTValue::Integer(1)],
        },
    ];

    let forms = variants
        .iter()
        .map(|value| semantic_canonical_form_bounded(value, &profile).unwrap())
        .collect::<Vec<_>>();
    for form in &forms[1..] {
        assert_eq!(form.value_id, forms[0].value_id);
        assert_eq!(
            form.canonical_serialization,
            forms[0].canonical_serialization
        );
    }
}

#[test]
fn dyadic_order_is_exact_beyond_floating_point_range() {
    let larger_tiny = DyadicRational::new(1, 200);
    let smaller_tiny = DyadicRational::new(1, 201);
    assert!(larger_tiny > smaller_tiny);
    assert!(smaller_tiny.negate() > larger_tiny.negate());

    let positive_switch = CGTValue::GameTree {
        left: vec![CGTValue::Dyadic(1, 200)],
        right: vec![CGTValue::Dyadic(1, 201)],
    };
    let negative_switch = CGTValue::GameTree {
        left: vec![CGTValue::Dyadic(-1, 201)],
        right: vec![CGTValue::Dyadic(-1, 200)],
    };
    assert_eq!(positive_switch.value_class(), ExactValueClass::Switch);
    assert_eq!(negative_switch.value_class(), ExactValueClass::Switch);
}

#[test]
fn checked_dyadic_arithmetic_rejects_unrepresentable_results() {
    let maximum = DyadicRational::new(i32::MAX, 0);
    let minimum = DyadicRational::new(i32::MIN, 0);
    let one = DyadicRational::new(1, 0);

    assert_eq!(maximum.checked_add(&one), None);
    assert_eq!(minimum.checked_sub(&one), None);
    assert_eq!(minimum.checked_negate(), None);
    assert_eq!(DyadicRational::new(1, 127).checked_add(&one), None);
}

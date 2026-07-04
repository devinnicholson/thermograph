use thermograph::{CGTValue, DyadicRational};

#[test]
fn dyadic_rational_addition_and_negation_are_exact() {
    let half = DyadicRational::new(1, 1);
    let quarter = DyadicRational::new(1, 2);

    assert_eq!(half.add(&half), DyadicRational::new(1, 0));
    assert_eq!(quarter.sub(&half), DyadicRational::new(-1, 2));
    assert_eq!(half.negate(), DyadicRational::new(-1, 1));
    assert_eq!(
        DyadicRational::new(2, 1).add(&DyadicRational::new(-1, 0)),
        DyadicRational::new(0, 0),
    );
}

#[test]
fn cgt_value_exact_number_arithmetic_uses_numeric_atoms() {
    let one = CGTValue::Integer(1);
    let half = CGTValue::Dyadic(1, 1);
    let quarter = CGTValue::Dyadic(1, 2);

    assert_eq!(one.add(&half), CGTValue::Dyadic(3, 1));
    assert_eq!(one.sub(&half), CGTValue::Dyadic(1, 1));
    assert_eq!(quarter.negate(), CGTValue::Dyadic(-1, 2));

    let values = vec![one, half, CGTValue::Dyadic(-1, 2)];
    assert_eq!(CGTValue::sum_all(&values), CGTValue::Dyadic(5, 2));
    assert_eq!(CGTValue::sum_all(values), CGTValue::Dyadic(5, 2));
}

#[test]
fn cgt_value_addition_falls_back_to_structural_game_tree() {
    let sum = CGTValue::Integer(1).add(&CGTValue::Star);

    assert_eq!(
        sum,
        CGTValue::GameTree {
            left: vec![CGTValue::Star, CGTValue::Integer(1)],
            right: vec![CGTValue::Integer(1)],
        },
    );

    assert_eq!(
        sum.canonical_serialization(),
        "GameTree(L[Number(1/2^0),Star];R[Number(1/2^0)])",
    );
}

#[test]
fn cgt_value_negation_swaps_structural_options() {
    let game = CGTValue::GameTree {
        left: vec![CGTValue::Integer(1)],
        right: vec![CGTValue::Star],
    };

    assert_eq!(
        game.negate(),
        CGTValue::GameTree {
            left: vec![CGTValue::Star],
            right: vec![CGTValue::Integer(-1)],
        },
    );
    assert_eq!(CGTValue::Star.negate(), CGTValue::Star);
    assert_eq!(CGTValue::Up.negate(), CGTValue::Down);
    assert_eq!(CGTValue::Down.negate(), CGTValue::Up);
}

#[test]
fn simplify_preserves_atomic_values() {
    for value in [
        CGTValue::Integer(2),
        CGTValue::Dyadic(1, 1),
        CGTValue::Star,
        CGTValue::Up,
        CGTValue::Down,
    ] {
        assert_eq!(value.simplify(), value);
    }
}

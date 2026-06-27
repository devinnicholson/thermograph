use thermograph::{CGTValue, DyadicRational};

#[test]
fn exact_dyadic_accessor_canonicalizes_numeric_values() {
    let integer = CGTValue::Integer(4);
    let dyadic_integer = CGTValue::Dyadic(8, 1);
    let half = CGTValue::Dyadic(1, 1);
    let zero = CGTValue::Dyadic(0, 64);

    assert_eq!(integer.try_to_dyadic(), Some(DyadicRational::new(4, 0)));
    assert_eq!(
        dyadic_integer.try_to_dyadic(),
        Some(DyadicRational::new(4, 0))
    );
    assert_eq!(half.try_to_dyadic(), Some(DyadicRational::new(1, 1)));
    assert_eq!(zero.try_to_dyadic(), Some(DyadicRational::new(0, 0)));
    assert_eq!(CGTValue::Star.try_to_dyadic(), None);
}

#[test]
fn canonical_serialization_and_digest_are_stable_for_core_values() {
    let cases = [
        (CGTValue::Integer(7), "Number(7/2^0)", "97e464279d224621"),
        (CGTValue::Dyadic(1, 1), "Number(1/2^1)", "ae0c0157cfae6faa"),
        (CGTValue::Star, "Star", "d98b59251f065471"),
        (CGTValue::Up, "Up", "09313a07b5c3ab60"),
        (CGTValue::Down, "Down", "032c13736048bf35"),
        (
            CGTValue::GameTree {
                left: vec![CGTValue::Star, CGTValue::Integer(1)],
                right: vec![CGTValue::Down, CGTValue::Integer(-1)],
            },
            "GameTree(L[Number(1/2^0),Star];R[Down,Number(-1/2^0)])",
            "c45a64ff05afdb7a",
        ),
    ];

    for (value, expected_serialization, expected_digest) in cases {
        assert_eq!(value.canonical_serialization(), expected_serialization);
        assert_eq!(
            value.canonical_bytes(),
            expected_serialization.as_bytes().to_vec()
        );
        assert_eq!(value.stable_canonical_digest(), expected_digest);
    }
}

#[test]
fn canonical_identity_reduces_dyadic_numbers() {
    let integer = CGTValue::Integer(1);
    let unreduced_dyadic = CGTValue::Dyadic(2, 1);
    let more_unreduced_dyadic = CGTValue::Dyadic(4, 2);

    assert_eq!(
        integer.canonical_serialization(),
        unreduced_dyadic.canonical_serialization()
    );
    assert_eq!(
        integer.stable_canonical_digest(),
        more_unreduced_dyadic.stable_canonical_digest()
    );
}

#[test]
fn simple_game_tree_identity_is_option_order_independent() {
    let first = CGTValue::GameTree {
        left: vec![CGTValue::Star, CGTValue::Integer(1), CGTValue::Integer(1)],
        right: vec![CGTValue::Up, CGTValue::Integer(-1)],
    };
    let reordered = CGTValue::GameTree {
        left: vec![CGTValue::Integer(1), CGTValue::Star],
        right: vec![CGTValue::Integer(-1), CGTValue::Up, CGTValue::Up],
    };

    assert_eq!(
        first.canonical_serialization(),
        "GameTree(L[Number(1/2^0),Star];R[Number(-1/2^0),Up])"
    );
    assert_eq!(
        first.canonical_serialization(),
        reordered.canonical_serialization()
    );
    assert_eq!(
        first.stable_canonical_digest(),
        reordered.stable_canonical_digest()
    );
    assert_eq!(first.stable_canonical_digest(), "f020bccebcefa0bd");
}

#[test]
fn equal_cloned_values_have_the_same_stable_hash() {
    let value = CGTValue::GameTree {
        left: vec![CGTValue::Integer(1), CGTValue::Star],
        right: vec![CGTValue::Integer(-1), CGTValue::Down],
    };
    let cloned = value.clone();

    assert_eq!(value, cloned);
    assert_eq!(
        value.stable_canonical_hash(),
        cloned.stable_canonical_hash()
    );
    assert_eq!(
        value.stable_canonical_digest(),
        cloned.stable_canonical_digest()
    );
}

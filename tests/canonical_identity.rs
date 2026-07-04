use thermograph::{CGTValue, DyadicRational, ExactValueClass};

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
        (
            CGTValue::Integer(7),
            ExactValueClass::Number,
            "Number(7/2^0)",
            "97e464279d224621",
        ),
        (
            CGTValue::Dyadic(1, 1),
            ExactValueClass::Number,
            "Number(1/2^1)",
            "ae0c0157cfae6faa",
        ),
        (
            CGTValue::Star,
            ExactValueClass::Star,
            "Star",
            "d98b59251f065471",
        ),
        (CGTValue::Up, ExactValueClass::Up, "Up", "09313a07b5c3ab60"),
        (
            CGTValue::Down,
            ExactValueClass::Down,
            "Down",
            "032c13736048bf35",
        ),
        (
            CGTValue::GameTree {
                left: vec![CGTValue::Integer(1)],
                right: vec![CGTValue::Integer(-1)],
            },
            ExactValueClass::Switch,
            "GameTree(L[Number(1/2^0)];R[Number(-1/2^0)])",
            "0cc26090a9cea850",
        ),
        (
            CGTValue::GameTree {
                left: vec![CGTValue::Star, CGTValue::Integer(1)],
                right: vec![CGTValue::Down, CGTValue::Integer(-1)],
            },
            ExactValueClass::GameTree,
            "GameTree(L[Number(1/2^0),Star];R[Down,Number(-1/2^0)])",
            "c45a64ff05afdb7a",
        ),
    ];

    for (value, expected_class, expected_serialization, expected_digest) in cases {
        assert_eq!(value.canonical_serialization(), expected_serialization);
        assert_eq!(
            value.canonical_bytes(),
            expected_serialization.as_bytes().to_vec()
        );
        assert_eq!(value.stable_canonical_digest(), expected_digest);

        let payload = value.exact_value_payload();
        assert_eq!(value.value_class(), expected_class);
        assert_eq!(payload.value_class, expected_class);
        assert_eq!(payload.canonical_serialization, expected_serialization);
        assert_eq!(payload.digest, expected_digest);
        assert_eq!(payload.dyadic, value.try_to_dyadic());
    }
}

#[test]
fn canonical_payload_v1_bytes_are_versioned_and_sha256_digest_is_stable() {
    let cases = [
        (
            CGTValue::Dyadic(1, 1),
            "Number(1/2^1)",
            "ae0c0157cfae6faa",
            "70fc7d1831b99b01d84159d7558ccef5b28efd1ab8071a283666e01c535dccbd",
        ),
        (
            CGTValue::Star,
            "Star",
            "d98b59251f065471",
            "3af68a355fca2793f09a34c7da705ba0078c03f063abe6ab5add40e65171b46f",
        ),
        (
            CGTValue::GameTree {
                left: vec![CGTValue::Integer(1)],
                right: vec![CGTValue::Integer(-1)],
            },
            "GameTree(L[Number(1/2^0)];R[Number(-1/2^0)])",
            "0cc26090a9cea850",
            "21b7aaee2441a3fad0609285136f142d46256899b8e5f50ce25c73c5797f3c94",
        ),
    ];

    for (value, expected_serialization, expected_stable_digest, expected_v1_digest) in cases {
        let mut expected_payload = b"thermograph.canonical_payload.v1\n".to_vec();
        expected_payload.extend_from_slice(expected_serialization.as_bytes());

        assert_eq!(value.canonical_payload_v1_bytes(), expected_payload);
        assert_eq!(value.digest_v1_sha256(), expected_v1_digest);
        assert_eq!(value.stable_canonical_digest(), expected_stable_digest);
        assert_eq!(value.canonical_bytes(), expected_serialization.as_bytes());
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

    for value in [integer, unreduced_dyadic, more_unreduced_dyadic] {
        let payload = value.exact_value_payload();

        assert_eq!(payload.value_class, ExactValueClass::Number);
        assert_eq!(payload.value_class.as_str(), "number");
        assert_eq!(payload.canonical_serialization, "Number(1/2^0)");
        assert_eq!(payload.digest, "ae089d57cfab8fe7");
        assert_eq!(payload.dyadic, Some(DyadicRational::new(1, 0)));
    }
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

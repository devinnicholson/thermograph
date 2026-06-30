mod fixtures {
    pub mod golden_values;
}

use fixtures::golden_values::golden_values;
use thermograph::{CGTValue, ExactValueClass};

const EPSILON: f32 = 1e-6;

#[derive(Debug, PartialEq, Eq)]
enum BmcomposeApproximateComposition {
    Unsupported,
}

#[derive(Debug, PartialEq, Eq)]
struct BmcomposeComponentContract {
    value_digest: String,
    approximate_composition: BmcomposeApproximateComposition,
}

fn bmcompose_component_contract(value: &CGTValue) -> BmcomposeComponentContract {
    let payload = value.exact_value_payload();

    BmcomposeComponentContract {
        value_digest: payload.digest,
        approximate_composition: BmcomposeApproximateComposition::Unsupported,
    }
}

fn assert_close(actual: f32, expected: f32, case_name: &str, field_name: &str) {
    assert!(
        (actual - expected).abs() <= EPSILON,
        "{case_name} {field_name}: expected {expected}, got {actual}",
    );
}

#[test]
fn golden_values_have_expected_options() {
    for case in golden_values() {
        let (left, right) = case.value.options();

        assert_eq!(
            left, case.expected_left,
            "{} left options changed",
            case.name,
        );
        assert_eq!(
            right, case.expected_right,
            "{} right options changed",
            case.name,
        );
    }
}

#[test]
fn golden_numeric_values_are_explicit() {
    for case in golden_values() {
        match case.expected_number {
            Some(expected) => {
                let actual = case
                    .value
                    .try_to_f32()
                    .unwrap_or_else(|| panic!("{} should be numeric", case.name));
                assert_close(actual, expected, case.name, "numeric value");
            }
            None => {
                assert!(
                    case.value.try_to_f32().is_none(),
                    "{} should remain non-numeric",
                    case.name,
                );
            }
        }
    }
}

#[test]
fn golden_exact_payload_marks_supported_dyadic_boundary() {
    for case in golden_values() {
        let payload = case.value.exact_value_payload();

        assert_eq!(
            payload.value_class,
            case.value.value_class(),
            "{} value class should come from the public contract",
            case.name,
        );
        assert_eq!(
            payload.canonical_serialization,
            case.value.canonical_serialization(),
            "{} canonical serialization changed",
            case.name,
        );
        assert_eq!(
            payload.digest,
            case.value.stable_canonical_digest(),
            "{} digest changed",
            case.name,
        );

        match case.value.try_to_dyadic() {
            Some(dyadic) => {
                assert_eq!(payload.value_class, ExactValueClass::Number);
                assert_eq!(
                    payload.dyadic,
                    Some(dyadic),
                    "{} should expose normalized exact dyadic data",
                    case.name,
                );
            }
            None => {
                assert_ne!(
                    payload.value_class,
                    ExactValueClass::Number,
                    "{} should not be classified as an exact number",
                    case.name,
                );
                assert_eq!(
                    payload.dyadic, None,
                    "{} should not expose unsupported exact dyadic data",
                    case.name,
                );
            }
        }
    }
}

#[test]
fn bmcompose_component_value_digest_uses_exact_payload_digest() {
    for case in golden_values() {
        let component = bmcompose_component_contract(&case.value);

        assert_eq!(
            component.value_digest,
            case.value.exact_value_payload().digest,
            "{} BMCOMPOSE component value_digest must come from exact_value_payload().digest",
            case.name,
        );
        assert_eq!(
            component.approximate_composition,
            BmcomposeApproximateComposition::Unsupported,
            "{} BMCOMPOSE approximate composition should be explicit when unsupported",
            case.name,
        );
    }
}

#[test]
fn bmcompose_payload_bytes_remain_stable_for_supported_component_classes() {
    let cases = [
        (
            "number",
            CGTValue::Dyadic(1, 1),
            ExactValueClass::Number,
            b"Number(1/2^1)".to_vec(),
            "ae0c0157cfae6faa",
        ),
        (
            "switch",
            CGTValue::GameTree {
                left: vec![CGTValue::Integer(1)],
                right: vec![CGTValue::Integer(-1)],
            },
            ExactValueClass::Switch,
            b"GameTree(L[Number(1/2^0)];R[Number(-1/2^0)])".to_vec(),
            "0cc26090a9cea850",
        ),
        (
            "game_tree",
            CGTValue::GameTree {
                left: vec![CGTValue::Star, CGTValue::Integer(1)],
                right: vec![CGTValue::Down, CGTValue::Integer(-1)],
            },
            ExactValueClass::GameTree,
            b"GameTree(L[Number(1/2^0),Star];R[Down,Number(-1/2^0)])".to_vec(),
            "c45a64ff05afdb7a",
        ),
    ];

    for (name, value, expected_class, expected_bytes, expected_digest) in cases {
        let payload = value.exact_value_payload();

        assert_eq!(
            payload.value_class, expected_class,
            "{name} value class changed",
        );
        assert_eq!(
            value.canonical_bytes(),
            expected_bytes,
            "{name} canonical payload bytes changed",
        );
        assert_eq!(
            payload.digest, expected_digest,
            "{name} exact payload digest changed",
        );
        assert_eq!(
            bmcompose_component_contract(&value).value_digest,
            expected_digest,
            "{name} BMCOMPOSE value_digest changed",
        );
    }
}

#[test]
fn golden_thermograph_mean_does_not_imply_exact_numeric_value() {
    let hot_case = golden_values()
        .into_iter()
        .find(|case| case.name == "hot_one_minus_one")
        .expect("hot_one_minus_one fixture should exist");
    let (temperature, mean) = hot_case.value.thermograph();
    let payload = hot_case.value.exact_value_payload();

    assert_close(temperature, 1.0, hot_case.name, "temperature");
    assert_close(mean, 0.0, hot_case.name, "mean");
    assert_eq!(hot_case.value.try_to_f32(), None);
    assert_eq!(payload.value_class, ExactValueClass::Switch);
    assert_eq!(payload.dyadic, None);
}

#[test]
fn golden_values_have_expected_thermographs() {
    for case in golden_values() {
        let (temperature, mean) = case.value.thermograph();

        assert_close(
            temperature,
            case.expected_temperature,
            case.name,
            "temperature",
        );
        assert_close(mean, case.expected_mean, case.name, "mean");
    }
}

#[test]
fn simplify_is_idempotent_for_supported_golden_values() {
    for case in golden_values()
        .into_iter()
        .filter(|case| case.simplify_idempotent)
    {
        let simplified = case.value.simplify();
        let simplified_again = simplified.simplify();

        assert_eq!(
            simplified, simplified_again,
            "{} simplify should be idempotent after the first pass",
            case.name,
        );

        if case.simplify_preserves_thermograph {
            let (temperature, mean) = simplified.thermograph();
            assert_close(
                temperature,
                case.expected_temperature,
                case.name,
                "simplified temperature",
            );
            assert_close(mean, case.expected_mean, case.name, "simplified mean");
        }
    }
}

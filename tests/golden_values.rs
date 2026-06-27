mod fixtures {
    pub mod golden_values;
}

use fixtures::golden_values::golden_values;

const EPSILON: f32 = 1e-6;

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

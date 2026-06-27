use thermograph::CGTValue;

#[derive(Debug)]
pub struct GoldenValue {
    pub name: &'static str,
    pub value: CGTValue,
    pub expected_left: Vec<CGTValue>,
    pub expected_right: Vec<CGTValue>,
    pub expected_number: Option<f32>,
    pub expected_temperature: f32,
    pub expected_mean: f32,
    pub simplify_idempotent: bool,
    pub simplify_preserves_thermograph: bool,
}

pub fn golden_values() -> Vec<GoldenValue> {
    vec![
        GoldenValue {
            name: "integer_two",
            value: CGTValue::Integer(2),
            expected_left: vec![CGTValue::Integer(1)],
            expected_right: vec![],
            expected_number: Some(2.0),
            expected_temperature: -1.0,
            expected_mean: 2.0,
            simplify_idempotent: true,
            simplify_preserves_thermograph: true,
        },
        GoldenValue {
            name: "dyadic_one_half",
            value: CGTValue::Dyadic(1, 1),
            expected_left: vec![CGTValue::Integer(0)],
            expected_right: vec![CGTValue::Integer(1)],
            expected_number: Some(0.5),
            expected_temperature: -1.0,
            expected_mean: 0.5,
            simplify_idempotent: true,
            // Current simplify expands dyadics into game trees whose f32 thermographs are hotter.
            simplify_preserves_thermograph: false,
        },
        GoldenValue {
            name: "star",
            value: CGTValue::Star,
            expected_left: vec![CGTValue::Integer(0)],
            expected_right: vec![CGTValue::Integer(0)],
            expected_number: None,
            expected_temperature: 0.0,
            expected_mean: 0.0,
            simplify_idempotent: true,
            simplify_preserves_thermograph: true,
        },
        GoldenValue {
            name: "up",
            value: CGTValue::Up,
            expected_left: vec![CGTValue::Integer(0)],
            expected_right: vec![CGTValue::Star],
            expected_number: None,
            expected_temperature: 0.0,
            expected_mean: 0.0,
            simplify_idempotent: true,
            simplify_preserves_thermograph: true,
        },
        GoldenValue {
            name: "down",
            value: CGTValue::Down,
            expected_left: vec![CGTValue::Star],
            expected_right: vec![CGTValue::Integer(0)],
            expected_number: None,
            expected_temperature: 0.0,
            expected_mean: 0.0,
            simplify_idempotent: true,
            simplify_preserves_thermograph: true,
        },
        GoldenValue {
            name: "hot_one_minus_one",
            value: CGTValue::GameTree {
                left: vec![CGTValue::Integer(1)],
                right: vec![CGTValue::Integer(-1)],
            },
            expected_left: vec![CGTValue::Integer(1)],
            expected_right: vec![CGTValue::Integer(-1)],
            expected_number: None,
            expected_temperature: 1.0,
            expected_mean: 0.0,
            simplify_idempotent: true,
            simplify_preserves_thermograph: true,
        },
    ]
}

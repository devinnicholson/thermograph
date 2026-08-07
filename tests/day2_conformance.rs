//! Exhaustive, dependency-free cross-runtime identity fixture through day two.

use std::collections::BTreeMap;

use thermograph::CGTValue;
use thermograph::short_game::{ShortGameProfile, semantic_canonical_form_bounded};

fn subsets(values: &[CGTValue]) -> Vec<Vec<CGTValue>> {
    (0_usize..(1_usize << values.len()))
        .map(|mask| {
            values
                .iter()
                .enumerate()
                .filter(|(index, _)| mask & (1 << index) != 0)
                .map(|(_, value)| value.clone())
                .collect()
        })
        .collect()
}

fn cumulative_next_day(
    previous: &[CGTValue],
    profile: &ShortGameProfile,
) -> BTreeMap<String, CGTValue> {
    let option_sets = subsets(previous);
    let mut values = BTreeMap::new();
    for left in &option_sets {
        for right in &option_sets {
            let form = semantic_canonical_form_bounded(
                &CGTValue::GameTree {
                    left: left.clone(),
                    right: right.clone(),
                },
                profile,
            )
            .expect("day-two fixture must fit the named profile");
            values.entry(form.value_id).or_insert(form.canonical_game);
        }
    }
    values
}

#[test]
fn all_256_day_two_literal_games_reduce_to_the_frozen_22_value_id_set() {
    let profile = ShortGameProfile::order7_v1();
    let day_zero = vec![CGTValue::Integer(0)];
    let day_one = cumulative_next_day(&day_zero, &profile);
    assert_eq!(day_one.len(), 4);

    let day_one_values = day_one.into_values().collect::<Vec<_>>();
    assert_eq!(subsets(&day_one_values).len().pow(2), 256);
    let day_two = cumulative_next_day(&day_one_values, &profile);
    let actual = day_two.keys().map(String::as_str).collect::<Vec<_>>();
    let expected = include_str!("../conformance/day2-semantic-ids-v1.txt")
        .lines()
        .collect::<Vec<_>>();

    assert_eq!(actual.len(), 22);
    assert_eq!(actual, expected);
}

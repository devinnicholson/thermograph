use std::collections::HashSet;

use thermograph::CGTValue;
use thermograph::short_game::{
    ShortGameComparison, ShortGameError, ShortGameProfile, ShortGameResource,
    compare_short_game_bounded, equal_short_game_bounded, expand_short_game_bounded,
    semantic_canonical_form_bounded, semantic_canonical_id_v1,
    semantic_target_catalogue_birthday3_bounded,
};

fn tree(left: Vec<CGTValue>, right: Vec<CGTValue>) -> CGTValue {
    CGTValue::GameTree { left, right }
}

#[test]
fn named_profiles_match_the_locked_resource_contract() {
    let order7 = ShortGameProfile::order7_v1();
    assert_eq!(order7.profile_id, "partizan.bounded_short_game.order7.v1");
    assert_eq!(order7.max_literal_birthday, 7);
    assert_eq!(order7.max_source_nodes, 128);
    assert_eq!(order7.max_options_per_side, 7);
    assert_eq!(order7.max_option_references, 1_792);
    assert_eq!(order7.max_intermediate_nodes, 4_096);
    assert_eq!(order7.max_rewrite_steps, 4_096);
    assert_eq!(order7.max_comparison_pairs, 262_144);
    assert_eq!(order7.max_serialization_bytes, 16 * 1_024 * 1_024);
    assert_eq!(order7.max_certificate_bytes, 64 * 1_024 * 1_024);

    let digraph8 = ShortGameProfile::digraph8_v1();
    assert_eq!(
        digraph8.profile_id,
        "partizan.bounded_short_game.digraph8.v1"
    );
    assert_eq!(digraph8.max_literal_birthday, 8);
    assert_eq!(digraph8.max_source_nodes, 256);
    assert_eq!(digraph8.max_options_per_side, 8);
    assert_eq!(digraph8.max_option_references, 4_096);
    assert_eq!(digraph8.max_intermediate_nodes, 8_192);
    assert_eq!(digraph8.max_rewrite_steps, 8_192);
    assert_eq!(digraph8.max_comparison_pairs, 1_000_000);
    assert_eq!(digraph8.max_serialization_bytes, 32 * 1_024 * 1_024);
    assert_eq!(digraph8.max_certificate_bytes, 128 * 1_024 * 1_024);
    assert_eq!(ShortGameProfile::default(), order7);
}

#[test]
fn expansion_erases_constructor_spelling_and_has_locked_digests() {
    let profile = ShortGameProfile::order7_v1();
    let explicit_zero = expand_short_game_bounded(&tree(vec![], vec![]), &profile).unwrap();
    assert_eq!(explicit_zero.birthday, 0);
    assert_eq!(explicit_zero.literal_serialization, "{|}");

    let nested_numeric =
        expand_short_game_bounded(&tree(vec![CGTValue::Integer(2)], vec![]), &profile).unwrap();
    assert_eq!(nested_numeric.birthday, 3);

    let atom = expand_short_game_bounded(&CGTValue::Star, &profile).unwrap();
    let explicit = expand_short_game_bounded(
        &tree(vec![CGTValue::Integer(0)], vec![CGTValue::Integer(0)]),
        &profile,
    )
    .unwrap();

    assert_eq!(atom.literal_serialization, "{{|}|{|}}");
    assert_eq!(atom.literal_serialization, explicit.literal_serialization);
    assert_eq!(atom.legacy_literal_sha256, explicit.legacy_literal_sha256);
    assert_eq!(atom.literal_sha256_v1, explicit.literal_sha256_v1);
    assert_ne!(atom.legacy_literal_sha256, atom.literal_sha256_v1);
    assert_eq!(atom.birthday, 1);
    assert!(matches!(atom.explicit_game, CGTValue::GameTree { .. }));
}

#[test]
fn option_order_and_duplicates_have_set_semantics() {
    let profile = ShortGameProfile::order7_v1();
    let first = tree(
        vec![CGTValue::Star, CGTValue::Integer(0), CGTValue::Star],
        vec![CGTValue::Integer(1)],
    );
    let second = tree(
        vec![CGTValue::Integer(0), CGTValue::Star],
        vec![CGTValue::Integer(1), CGTValue::Integer(1)],
    );
    let first = expand_short_game_bounded(&first, &profile).unwrap();
    let second = expand_short_game_bounded(&second, &profile).unwrap();
    assert_eq!(first.literal_serialization, second.literal_serialization);
    assert_eq!(first.literal_sha256_v1, second.literal_sha256_v1);
}

#[test]
fn atom_tree_and_elkies_equalities_are_exact() {
    let profile = ShortGameProfile::order7_v1();
    let zero_tree = tree(vec![], vec![]);
    let one_tree = tree(vec![CGTValue::Integer(0)], vec![]);
    let star_tree = tree(vec![CGTValue::Integer(0)], vec![CGTValue::Integer(0)]);
    let half_tree = tree(vec![CGTValue::Integer(0)], vec![CGTValue::Integer(1)]);
    let elkies = tree(
        vec![CGTValue::Integer(0), CGTValue::Star],
        vec![CGTValue::Integer(1)],
    );

    assert!(equal_short_game_bounded(&CGTValue::Integer(0), &zero_tree, &profile).unwrap());
    assert!(equal_short_game_bounded(&CGTValue::Integer(1), &one_tree, &profile).unwrap());
    assert!(equal_short_game_bounded(&CGTValue::Star, &star_tree, &profile).unwrap());
    assert!(equal_short_game_bounded(&CGTValue::Dyadic(1, 1), &half_tree, &profile).unwrap());
    assert!(equal_short_game_bounded(&CGTValue::Dyadic(1, 1), &elkies, &profile).unwrap());
}

#[test]
fn comparison_distinguishes_order_and_fuzziness() {
    let profile = ShortGameProfile::order7_v1();
    assert_eq!(
        compare_short_game_bounded(&CGTValue::Integer(-1), &CGTValue::Integer(0), &profile)
            .unwrap(),
        ShortGameComparison::Less
    );
    assert_eq!(
        compare_short_game_bounded(&CGTValue::Integer(1), &CGTValue::Integer(0), &profile).unwrap(),
        ShortGameComparison::Greater
    );
    assert_eq!(
        compare_short_game_bounded(&CGTValue::Star, &CGTValue::Integer(0), &profile).unwrap(),
        ShortGameComparison::Fuzzy
    );
    assert_eq!(
        compare_short_game_bounded(&CGTValue::Up, &CGTValue::Integer(0), &profile).unwrap(),
        ShortGameComparison::Greater
    );
    assert_eq!(
        compare_short_game_bounded(&CGTValue::Down, &CGTValue::Integer(0), &profile).unwrap(),
        ShortGameComparison::Less
    );
}

#[test]
fn domination_and_reversibility_preserve_exact_value() {
    let profile = ShortGameProfile::order7_v1();
    let dominated = tree(
        vec![CGTValue::Integer(1), CGTValue::Integer(0)],
        vec![CGTValue::Integer(-1), CGTValue::Integer(0)],
    );
    let switch = tree(vec![CGTValue::Integer(1)], vec![CGTValue::Integer(-1)]);
    let reversible_left = tree(vec![CGTValue::Star], vec![]);
    let reversible_right = tree(vec![], vec![CGTValue::Star]);

    assert!(equal_short_game_bounded(&dominated, &switch, &profile).unwrap());
    assert!(equal_short_game_bounded(&reversible_left, &CGTValue::Integer(0), &profile).unwrap());
    assert!(equal_short_game_bounded(&reversible_right, &CGTValue::Integer(0), &profile).unwrap());
}

#[test]
fn birthday_and_side_limits_return_typed_errors() {
    let mut profile = ShortGameProfile::order7_v1();
    profile.max_literal_birthday = 1;
    let error = expand_short_game_bounded(&CGTValue::Integer(2), &profile).unwrap_err();
    assert!(matches!(
        error,
        ShortGameError::ResourceLimit(ref limit)
            if limit.resource == ShortGameResource::LiteralBirthday
                && limit.limit == 1
                && limit.observed == 2
    ));

    profile.max_literal_birthday = 7;
    profile.max_options_per_side = 1;
    let error = expand_short_game_bounded(
        &tree(vec![CGTValue::Integer(0), CGTValue::Star], Vec::new()),
        &profile,
    )
    .unwrap_err();
    assert!(matches!(
        error,
        ShortGameError::ResourceLimit(ref limit)
            if limit.resource == ShortGameResource::OptionsPerSide
    ));
}

#[test]
fn source_node_and_comparison_limits_return_no_verdict() {
    let mut profile = ShortGameProfile::order7_v1();
    profile.max_source_nodes = 1;
    let error = compare_short_game_bounded(&CGTValue::Integer(0), &CGTValue::Integer(1), &profile)
        .unwrap_err();
    assert!(matches!(
        error,
        ShortGameError::ResourceLimit(ref limit)
            if limit.resource == ShortGameResource::SourceNodes
    ));

    profile.max_source_nodes = 128;
    profile.max_comparison_pairs = 0;
    let error = compare_short_game_bounded(&CGTValue::Integer(0), &CGTValue::Integer(0), &profile)
        .unwrap_err();
    assert!(matches!(
        error,
        ShortGameError::ResourceLimit(ref limit)
            if limit.resource == ShortGameResource::ComparisonPairs
    ));
}

#[test]
fn source_node_limit_is_per_root_while_the_arena_is_combined() {
    let mut profile = ShortGameProfile::order7_v1();
    profile.max_source_nodes = 2;
    profile.max_intermediate_nodes = 3;
    assert_eq!(
        compare_short_game_bounded(&CGTValue::Star, &CGTValue::Integer(1), &profile).unwrap(),
        ShortGameComparison::Less
    );
}

#[test]
fn semantic_ids_coalesce_atoms_and_explicit_definitions() {
    let profile = ShortGameProfile::order7_v1();
    let pairs = [
        (CGTValue::Integer(0), tree(vec![], vec![])),
        (
            CGTValue::Integer(1),
            tree(vec![CGTValue::Integer(0)], vec![]),
        ),
        (
            CGTValue::Star,
            tree(vec![CGTValue::Integer(0)], vec![CGTValue::Integer(0)]),
        ),
    ];
    for (atom, explicit) in pairs {
        let atom = semantic_canonical_form_bounded(&atom, &profile).unwrap();
        let explicit = semantic_canonical_form_bounded(&explicit, &profile).unwrap();
        assert_eq!(
            atom.canonical_serialization,
            explicit.canonical_serialization
        );
        assert_eq!(atom.value_id, explicit.value_id);
        assert_eq!(
            atom.value_id,
            semantic_canonical_id_v1(&atom.canonical_serialization)
        );
    }
}

#[test]
fn elkies_domination_and_reversibility_reduce_deterministically() {
    let profile = ShortGameProfile::order7_v1();
    let elkies = tree(
        vec![CGTValue::Integer(0), CGTValue::Star],
        vec![CGTValue::Integer(1)],
    );
    let half = semantic_canonical_form_bounded(&CGTValue::Dyadic(1, 1), &profile).unwrap();
    let elkies = semantic_canonical_form_bounded(&elkies, &profile).unwrap();
    assert_eq!(elkies.value_id, half.value_id);
    assert!(elkies.rewrite_steps > 0);

    let dominated = tree(
        vec![CGTValue::Integer(1), CGTValue::Integer(0)],
        vec![CGTValue::Integer(-1), CGTValue::Integer(0)],
    );
    let switch = tree(vec![CGTValue::Integer(1)], vec![CGTValue::Integer(-1)]);
    assert_eq!(
        semantic_canonical_form_bounded(&dominated, &profile)
            .unwrap()
            .value_id,
        semantic_canonical_form_bounded(&switch, &profile)
            .unwrap()
            .value_id
    );

    for reversible in [
        tree(vec![CGTValue::Star], vec![]),
        tree(vec![], vec![CGTValue::Star]),
    ] {
        assert_eq!(
            semantic_canonical_form_bounded(&reversible, &profile)
                .unwrap()
                .value_id,
            semantic_canonical_form_bounded(&CGTValue::Integer(0), &profile)
                .unwrap()
                .value_id
        );
    }
}

#[test]
fn additive_cancellations_reduce_to_zero() {
    let profile = ShortGameProfile::order7_v1();
    let zero = semantic_canonical_form_bounded(&CGTValue::Integer(0), &profile)
        .unwrap()
        .value_id;
    assert_eq!(
        semantic_canonical_form_bounded(&CGTValue::Star.add(&CGTValue::Star), &profile)
            .unwrap()
            .value_id,
        zero
    );
    assert_eq!(
        semantic_canonical_form_bounded(&CGTValue::Up.add(&CGTValue::Down), &profile)
            .unwrap()
            .value_id,
        zero
    );
}

#[test]
fn canonical_output_is_idempotent_and_unequal_games_keep_distinct_ids() {
    let profile = ShortGameProfile::order7_v1();
    let source = tree(
        vec![CGTValue::Integer(0), CGTValue::Star],
        vec![CGTValue::Integer(1)],
    );
    let first = semantic_canonical_form_bounded(&source, &profile).unwrap();
    let second = semantic_canonical_form_bounded(&first.canonical_game, &profile).unwrap();
    assert_eq!(
        first.canonical_serialization,
        second.canonical_serialization
    );
    assert_eq!(first.value_id, second.value_id);
    assert_ne!(
        semantic_canonical_form_bounded(&CGTValue::Star, &profile)
            .unwrap()
            .value_id,
        semantic_canonical_form_bounded(&CGTValue::Integer(0), &profile)
            .unwrap()
            .value_id
    );
}

#[test]
fn canonical_resource_limits_return_typed_failures() {
    let mut profile = ShortGameProfile::order7_v1();
    profile.max_canonical_birthday = 0;
    let error = semantic_canonical_form_bounded(&CGTValue::Star, &profile).unwrap_err();
    assert!(matches!(
        error,
        ShortGameError::ResourceLimit(ref limit)
            if limit.resource == ShortGameResource::CanonicalBirthday
    ));

    profile.max_canonical_birthday = 7;
    profile.max_rewrite_steps = 0;
    let elkies = tree(
        vec![CGTValue::Integer(0), CGTValue::Star],
        vec![CGTValue::Integer(1)],
    );
    let error = semantic_canonical_form_bounded(&elkies, &profile).unwrap_err();
    assert!(matches!(
        error,
        ShortGameError::ResourceLimit(ref limit)
            if limit.resource == ShortGameResource::RewriteSteps
    ));
}

#[test]
fn birthday_three_catalogue_has_complete_known_counts_and_invariants() {
    let profile = ShortGameProfile::order7_v1();
    let catalogue = semantic_target_catalogue_birthday3_bounded(&profile).unwrap();
    assert_eq!(catalogue.maximum_birthday, 3);
    assert_eq!(catalogue.cumulative_counts, [1, 4, 22, 1_474]);
    assert_eq!(catalogue.exact_birthday_counts, [1, 3, 18, 1_452]);
    assert_eq!(catalogue.day2_antichain_count, 98);
    assert_eq!(catalogue.day3_candidate_pairs, 9_604);
    assert_eq!(catalogue.rows.len(), 1_474);

    let unique_ids = catalogue
        .rows
        .iter()
        .map(|row| row.value_id.as_str())
        .collect::<HashSet<_>>();
    assert_eq!(unique_ids.len(), catalogue.rows.len());
    assert!(catalogue.rows.iter().all(|row| row.birthday <= 3));
    assert!(catalogue.rows.windows(2).all(|rows| {
        (&rows[0].value_id, &rows[0].canonical_serialization)
            < (&rows[1].value_id, &rows[1].canonical_serialization)
    }));

    for row in &catalogue.rows {
        let repeated = semantic_canonical_form_bounded(&row.canonical_game, &profile).unwrap();
        assert_eq!(repeated.value_id, row.value_id);
        assert_eq!(
            repeated.canonical_serialization,
            row.canonical_serialization
        );
    }
}

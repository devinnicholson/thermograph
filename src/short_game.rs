//! Bounded exact operations for finite normal-play short games.
//!
//! This module expands every numeric or named [`CGTValue`] constructor into an
//! explicit option tree before computing literal identity or mathematical
//! comparison. Its identities are separate from the legacy structural
//! serialization and digest APIs on [`CGTValue`].

use std::collections::{BTreeMap, HashMap};
use std::fmt;

use crate::{CGTValue, DyadicRational};

const LITERAL_V1_PREFIX: &[u8] = b"partizan.explicit_short_game.v1\n";
const SEMANTIC_CANONICAL_V1_PREFIX: &[u8] = b"partizan.semantic_canonical_game.v1\n";

/// Resource profile for one bounded short-game operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ShortGameProfile {
    /// Stable profile identifier recorded with research artifacts.
    pub profile_id: &'static str,
    /// Maximum birthday of either input root.
    pub max_literal_birthday: u32,
    /// Maximum birthday reserved for a later canonical-form stage.
    pub max_canonical_birthday: u32,
    /// Maximum distinct expanded source nodes across all input roots.
    pub max_source_nodes: usize,
    /// Maximum distinct options on either side of any expanded node.
    pub max_options_per_side: usize,
    /// Maximum option references across the expanded source closure.
    pub max_option_references: usize,
    /// Maximum nodes reserved for source and future intermediate forms.
    pub max_intermediate_nodes: usize,
    /// Maximum canonical reduction rewrites in one request.
    pub max_rewrite_steps: usize,
    /// Maximum memoized ordered comparison pairs.
    pub max_comparison_pairs: usize,
    /// Maximum UTF-8 bytes in a root brace serialization.
    pub max_serialization_bytes: usize,
    /// Maximum certificate bytes reserved for a later certificate API.
    pub max_certificate_bytes: usize,
}

impl ShortGameProfile {
    /// Returns the profile for Partizan order-seven realization experiments.
    #[must_use]
    pub const fn order7_v1() -> Self {
        Self {
            profile_id: "partizan.bounded_short_game.order7.v1",
            max_literal_birthday: 7,
            max_canonical_birthday: 7,
            max_source_nodes: 128,
            max_options_per_side: 7,
            max_option_references: 1_792,
            max_intermediate_nodes: 4_096,
            max_rewrite_steps: 4_096,
            max_comparison_pairs: 262_144,
            max_serialization_bytes: 16 * 1_024 * 1_024,
            max_certificate_bytes: 64 * 1_024 * 1_024,
        }
    }

    /// Returns the profile for Partizan eight-vertex digraph experiments.
    #[must_use]
    pub const fn digraph8_v1() -> Self {
        Self {
            profile_id: "partizan.bounded_short_game.digraph8.v1",
            max_literal_birthday: 8,
            max_canonical_birthday: 8,
            max_source_nodes: 256,
            max_options_per_side: 8,
            max_option_references: 4_096,
            max_intermediate_nodes: 8_192,
            max_rewrite_steps: 8_192,
            max_comparison_pairs: 1_000_000,
            max_serialization_bytes: 32 * 1_024 * 1_024,
            max_certificate_bytes: 128 * 1_024 * 1_024,
        }
    }
}

impl Default for ShortGameProfile {
    fn default() -> Self {
        Self::order7_v1()
    }
}

/// A resource governed by [`ShortGameProfile`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShortGameResource {
    /// Birthday of an input root.
    LiteralBirthday,
    /// Distinct nodes in the expanded source closure.
    SourceNodes,
    /// Distinct nodes in the combined interned arena.
    IntermediateNodes,
    /// Birthday of the reduced semantic canonical form.
    CanonicalBirthday,
    /// Options on one side of one expanded node.
    OptionsPerSide,
    /// Option references in the expanded source closure.
    OptionReferences,
    /// Memoized ordered comparison pairs.
    ComparisonPairs,
    /// Canonical reduction rewrites.
    RewriteSteps,
    /// UTF-8 bytes in a root brace serialization.
    SerializationBytes,
}

impl ShortGameResource {
    fn as_str(self) -> &'static str {
        match self {
            Self::LiteralBirthday => "literal_birthday",
            Self::SourceNodes => "source_nodes",
            Self::IntermediateNodes => "intermediate_nodes",
            Self::CanonicalBirthday => "canonical_birthday",
            Self::OptionsPerSide => "options_per_side",
            Self::OptionReferences => "option_references",
            Self::ComparisonPairs => "comparison_pairs",
            Self::RewriteSteps => "rewrite_steps",
            Self::SerializationBytes => "serialization_bytes",
        }
    }
}

/// Details of a bounded operation that exceeded its profile.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResourceLimitError {
    /// Resource whose bound was crossed.
    pub resource: ShortGameResource,
    /// Configured inclusive limit.
    pub limit: u64,
    /// Observed value, or a saturated lower bound when the exact size overflowed.
    pub observed: u64,
}

/// Failure from a bounded short-game operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShortGameError {
    /// A named profile resource was exceeded.
    ResourceLimit(ResourceLimitError),
    /// A numeric constructor could not be expanded with checked arithmetic.
    ArithmeticOverflow,
    /// A semantic postcondition failed after canonical reduction.
    SemanticVerificationFailed,
}

impl fmt::Display for ShortGameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ResourceLimit(error) => write!(
                f,
                "resource_limit: {} observed {} exceeds {}",
                error.resource.as_str(),
                error.observed,
                error.limit
            ),
            Self::ArithmeticOverflow => {
                f.write_str("arithmetic_overflow while expanding numeric constructor")
            }
            Self::SemanticVerificationFailed => {
                f.write_str("semantic_verification_failed after canonical reduction")
            }
        }
    }
}

impl std::error::Error for ShortGameError {}

/// Four-way Conway order relation between two short games.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShortGameComparison {
    /// The left game is strictly less than the right game.
    Less,
    /// The two games are mathematically equal.
    Equal,
    /// The left game is strictly greater than the right game.
    Greater,
    /// The games are fuzzy, so neither is less than or equal to the other.
    Fuzzy,
}

/// An atom-free explicit expansion and its two literal SHA-256 identities.
#[derive(Debug, Clone, PartialEq)]
pub struct ExpandedShortGame {
    /// Explicit atom-free root; every node is [`CGTValue::GameTree`].
    pub explicit_game: CGTValue,
    /// Deterministic brace serialization with set-valued option semantics.
    pub literal_serialization: String,
    /// Legacy unprefixed `sha256(literal_serialization)` used by frozen work.
    pub legacy_literal_sha256: String,
    /// Domain-separated version-one literal SHA-256 identifier.
    pub literal_sha256_v1: String,
    /// Birthday of the explicit root.
    pub birthday: u32,
    /// Distinct nodes in the interned expanded closure.
    pub distinct_subgames: usize,
    /// Option references in the interned expanded closure.
    pub option_references: usize,
}

/// A bounded semantic canonical form and its domain-separated value identity.
#[derive(Debug, Clone, PartialEq)]
pub struct SemanticCanonicalForm {
    /// Canonical atom-free explicit root.
    pub canonical_game: CGTValue,
    /// Deterministic brace serialization of the reduced root.
    pub canonical_serialization: String,
    /// `sha256("partizan.semantic_canonical_game.v1\n" + serialization)`.
    pub value_id: String,
    /// Birthday of the reduced root.
    pub canonical_birthday: u32,
    /// Number of deterministic reduction rewrites.
    pub rewrite_steps: usize,
    /// Number of memoized ordered comparison pairs used by reduction and audit.
    pub comparison_pairs: usize,
    /// Total distinct nodes interned across source and intermediate forms.
    pub intermediate_nodes: usize,
}

/// One canonical value in the complete birthday-three semantic catalogue.
#[derive(Debug, Clone, PartialEq)]
pub struct SemanticTargetCatalogueRow {
    /// Canonical atom-free explicit root.
    pub canonical_game: CGTValue,
    /// Canonical brace serialization.
    pub canonical_serialization: String,
    /// Domain-separated semantic canonical value identifier.
    pub value_id: String,
    /// Exact canonical birthday, in the range zero through three.
    pub birthday: u32,
}

/// Complete deterministic catalogue of normal-play values born by day three.
#[derive(Debug, Clone, PartialEq)]
pub struct SemanticTargetCatalogue {
    /// Profile used for every bounded comparison and canonicalization.
    pub profile_id: &'static str,
    /// Largest canonical birthday included.
    pub maximum_birthday: u32,
    /// Rows sorted by value identifier and then canonical serialization.
    pub rows: Vec<SemanticTargetCatalogueRow>,
    /// Cumulative value counts for birthdays zero through three.
    pub cumulative_counts: [usize; 4],
    /// Exact value counts born on days zero through three.
    pub exact_birthday_counts: [usize; 4],
    /// Number of antichains in the cumulative birthday-two order.
    pub day2_antichain_count: usize,
    /// Number of ordered Left/Right antichain pairs examined for day three.
    pub day3_candidate_pairs: usize,
}

type NodeId = usize;

#[derive(Debug, Clone)]
struct Node {
    left: Vec<NodeId>,
    right: Vec<NodeId>,
    birthday: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct NodeKey {
    left: Vec<NodeId>,
    right: Vec<NodeId>,
}

struct Arena<'a> {
    profile: &'a ShortGameProfile,
    nodes: Vec<Node>,
    interned: HashMap<NodeKey, NodeId>,
    option_references: usize,
    serialization_cache: HashMap<NodeId, String>,
}

impl<'a> Arena<'a> {
    fn new(profile: &'a ShortGameProfile) -> Self {
        Self {
            profile,
            nodes: Vec::new(),
            interned: HashMap::new(),
            option_references: 0,
            serialization_cache: HashMap::new(),
        }
    }

    fn resource(
        &self,
        resource: ShortGameResource,
        limit: usize,
        observed: usize,
    ) -> ShortGameError {
        ShortGameError::ResourceLimit(ResourceLimitError {
            resource,
            limit: u64::try_from(limit).unwrap_or(u64::MAX),
            observed: u64::try_from(observed).unwrap_or(u64::MAX),
        })
    }

    fn intern(
        &mut self,
        mut left: Vec<NodeId>,
        mut right: Vec<NodeId>,
    ) -> Result<NodeId, ShortGameError> {
        left.sort_unstable();
        left.dedup();
        right.sort_unstable();
        right.dedup();

        let larger_side = left.len().max(right.len());
        if larger_side > self.profile.max_options_per_side {
            return Err(self.resource(
                ShortGameResource::OptionsPerSide,
                self.profile.max_options_per_side,
                larger_side,
            ));
        }

        let key = NodeKey {
            left: left.clone(),
            right: right.clone(),
        };
        if let Some(id) = self.interned.get(&key) {
            return Ok(*id);
        }

        let next_nodes = self.nodes.len().saturating_add(1);
        if next_nodes > self.profile.max_intermediate_nodes {
            return Err(self.resource(
                ShortGameResource::IntermediateNodes,
                self.profile.max_intermediate_nodes,
                next_nodes,
            ));
        }

        let added_references = left.len().saturating_add(right.len());
        let next_references = self.option_references.saturating_add(added_references);
        if next_references > self.profile.max_option_references {
            return Err(self.resource(
                ShortGameResource::OptionReferences,
                self.profile.max_option_references,
                next_references,
            ));
        }

        let birthday = left
            .iter()
            .chain(&right)
            .map(|id| self.nodes[*id].birthday)
            .max()
            .map_or(0, |birthday| birthday.saturating_add(1));
        let id = self.nodes.len();
        self.nodes.push(Node {
            left,
            right,
            birthday,
        });
        self.interned.insert(key, id);
        self.option_references = next_references;
        Ok(id)
    }

    fn reachable_node_count(&self, root: NodeId) -> usize {
        let mut seen = vec![false; self.nodes.len()];
        let mut stack = vec![root];
        let mut count = 0;
        while let Some(id) = stack.pop() {
            if seen[id] {
                continue;
            }
            seen[id] = true;
            count += 1;
            stack.extend(self.nodes[id].left.iter().copied());
            stack.extend(self.nodes[id].right.iter().copied());
        }
        count
    }

    fn enforce_source_node_limit(&self, root: NodeId) -> Result<(), ShortGameError> {
        let observed = self.reachable_node_count(root);
        if observed > self.profile.max_source_nodes {
            return Err(self.resource(
                ShortGameResource::SourceNodes,
                self.profile.max_source_nodes,
                observed,
            ));
        }
        Ok(())
    }

    fn expand(&mut self, value: &CGTValue) -> Result<NodeId, ShortGameError> {
        match value {
            CGTValue::Integer(integer) => self.expand_number(i64::from(*integer), 0),
            CGTValue::Dyadic(numerator, denominator_power) => {
                let dyadic = DyadicRational::new(*numerator, *denominator_power);
                self.expand_number(i64::from(dyadic.numerator()), dyadic.denominator_power())
            }
            CGTValue::Star => {
                let zero = self.expand_number(0, 0)?;
                self.intern(vec![zero], vec![zero])
            }
            CGTValue::Up => {
                let zero = self.expand_number(0, 0)?;
                let star = self.expand(&CGTValue::Star)?;
                self.intern(vec![zero], vec![star])
            }
            CGTValue::Down => {
                let zero = self.expand_number(0, 0)?;
                let star = self.expand(&CGTValue::Star)?;
                self.intern(vec![star], vec![zero])
            }
            CGTValue::GameTree { left, right } => {
                let expanded_left = left
                    .iter()
                    .map(|option| self.expand(option))
                    .collect::<Result<Vec<_>, _>>()?;
                let expanded_right = right
                    .iter()
                    .map(|option| self.expand(option))
                    .collect::<Result<Vec<_>, _>>()?;
                self.intern(expanded_left, expanded_right)
            }
        }
    }

    fn expand_number(
        &mut self,
        mut numerator: i64,
        mut denominator_power: u32,
    ) -> Result<NodeId, ShortGameError> {
        if numerator == 0 {
            denominator_power = 0;
        }
        while denominator_power > 0 && numerator % 2 == 0 {
            numerator /= 2;
            denominator_power -= 1;
        }

        if denominator_power == 0 {
            return match numerator.cmp(&0) {
                std::cmp::Ordering::Equal => self.intern(Vec::new(), Vec::new()),
                std::cmp::Ordering::Greater => {
                    let predecessor = numerator
                        .checked_sub(1)
                        .ok_or(ShortGameError::ArithmeticOverflow)?;
                    let left = self.expand_number(predecessor, 0)?;
                    self.intern(vec![left], Vec::new())
                }
                std::cmp::Ordering::Less => {
                    let successor = numerator
                        .checked_add(1)
                        .ok_or(ShortGameError::ArithmeticOverflow)?;
                    let right = self.expand_number(successor, 0)?;
                    self.intern(Vec::new(), vec![right])
                }
            };
        }

        let left_numerator = numerator
            .checked_sub(1)
            .ok_or(ShortGameError::ArithmeticOverflow)?;
        let right_numerator = numerator
            .checked_add(1)
            .ok_or(ShortGameError::ArithmeticOverflow)?;
        let left = self.expand_number(left_numerator, denominator_power)?;
        let right = self.expand_number(right_numerator, denominator_power)?;
        self.intern(vec![left], vec![right])
    }

    fn serialization(&mut self, id: NodeId) -> Result<String, ShortGameError> {
        if let Some(serialization) = self.serialization_cache.get(&id) {
            return Ok(serialization.clone());
        }
        let node = self.nodes[id].clone();
        let mut left = node
            .left
            .iter()
            .map(|option| self.serialization(*option))
            .collect::<Result<Vec<_>, _>>()?;
        let mut right = node
            .right
            .iter()
            .map(|option| self.serialization(*option))
            .collect::<Result<Vec<_>, _>>()?;
        left.sort();
        left.dedup();
        right.sort();
        right.dedup();

        let observed = 3_usize
            .saturating_add(left.iter().map(String::len).sum::<usize>())
            .saturating_add(right.iter().map(String::len).sum::<usize>())
            .saturating_add(left.len().saturating_sub(1))
            .saturating_add(right.len().saturating_sub(1));
        if observed > self.profile.max_serialization_bytes {
            return Err(self.resource(
                ShortGameResource::SerializationBytes,
                self.profile.max_serialization_bytes,
                observed,
            ));
        }
        let serialization = format!("{{{}|{}}}", left.join(","), right.join(","));
        self.serialization_cache.insert(id, serialization.clone());
        Ok(serialization)
    }

    fn explicit_value(&mut self, id: NodeId) -> Result<CGTValue, ShortGameError> {
        let node = self.nodes[id].clone();
        let mut left = node
            .left
            .iter()
            .map(|option| Ok((self.serialization(*option)?, self.explicit_value(*option)?)))
            .collect::<Result<Vec<_>, ShortGameError>>()?;
        let mut right = node
            .right
            .iter()
            .map(|option| Ok((self.serialization(*option)?, self.explicit_value(*option)?)))
            .collect::<Result<Vec<_>, ShortGameError>>()?;
        left.sort_by(|a, b| a.0.cmp(&b.0));
        right.sort_by(|a, b| a.0.cmp(&b.0));
        Ok(CGTValue::GameTree {
            left: left.into_iter().map(|(_, value)| value).collect(),
            right: right.into_iter().map(|(_, value)| value).collect(),
        })
    }
}

fn limit_error(resource: ShortGameResource, limit: u32, observed: u64) -> ShortGameError {
    ShortGameError::ResourceLimit(ResourceLimitError {
        resource,
        limit: u64::from(limit),
        observed,
    })
}

fn numeric_birthday(numerator: i64, denominator_power: u32) -> u64 {
    let magnitude = numerator.unsigned_abs();
    if numerator == 0 {
        return 0;
    }
    if denominator_power == 0 {
        return magnitude;
    }
    let integer_ceiling = if denominator_power >= 64 {
        1
    } else {
        magnitude.saturating_add((1_u64 << denominator_power) - 1) >> denominator_power
    };
    integer_ceiling.saturating_add(u64::from(denominator_power))
}

fn preflight_birthday(value: &CGTValue, profile: &ShortGameProfile) -> Result<u32, ShortGameError> {
    fn visit(value: &CGTValue) -> Result<u64, ShortGameError> {
        Ok(match value {
            CGTValue::Integer(integer) => numeric_birthday(i64::from(*integer), 0),
            CGTValue::Dyadic(numerator, denominator_power) => {
                let dyadic = DyadicRational::new(*numerator, *denominator_power);
                numeric_birthday(i64::from(dyadic.numerator()), dyadic.denominator_power())
            }
            CGTValue::Star => 1,
            CGTValue::Up | CGTValue::Down => 2,
            CGTValue::GameTree { left, right } => left
                .iter()
                .chain(right)
                .map(visit)
                .collect::<Result<Vec<_>, _>>()?
                .into_iter()
                .max()
                .map_or(0_u64, |birthday| birthday.saturating_add(1)),
        })
    }

    let observed = visit(value)?;
    if observed > u64::from(profile.max_literal_birthday) {
        return Err(limit_error(
            ShortGameResource::LiteralBirthday,
            profile.max_literal_birthday,
            observed,
        ));
    }
    u32::try_from(observed).map_err(|_| ShortGameError::ArithmeticOverflow)
}

fn sha256_hex(bytes: &[u8]) -> String {
    crate::hex_lower(&crate::sha256_digest(bytes))
}

/// Expands all constructors into an explicit atom-free game under `profile`.
pub fn expand_short_game_bounded(
    value: &CGTValue,
    profile: &ShortGameProfile,
) -> Result<ExpandedShortGame, ShortGameError> {
    let birthday = preflight_birthday(value, profile)?;
    let mut arena = Arena::new(profile);
    let root = arena.expand(value)?;
    arena.enforce_source_node_limit(root)?;
    debug_assert_eq!(birthday, arena.nodes[root].birthday);
    let literal_serialization = arena.serialization(root)?;
    let legacy_literal_sha256 = sha256_hex(literal_serialization.as_bytes());
    let mut versioned_bytes =
        Vec::with_capacity(LITERAL_V1_PREFIX.len() + literal_serialization.len());
    versioned_bytes.extend_from_slice(LITERAL_V1_PREFIX);
    versioned_bytes.extend_from_slice(literal_serialization.as_bytes());
    let literal_sha256_v1 = sha256_hex(&versioned_bytes);
    let explicit_game = arena.explicit_value(root)?;
    Ok(ExpandedShortGame {
        explicit_game,
        literal_serialization,
        legacy_literal_sha256,
        literal_sha256_v1,
        birthday,
        distinct_subgames: arena.nodes.len(),
        option_references: arena.option_references,
    })
}

struct ComparisonEngine<'a> {
    arena: Arena<'a>,
    memo: HashMap<(NodeId, NodeId), bool>,
}

impl ComparisonEngine<'_> {
    fn less_or_equal(&mut self, left: NodeId, right: NodeId) -> Result<bool, ShortGameError> {
        if let Some(result) = self.memo.get(&(left, right)) {
            return Ok(*result);
        }
        let observed = self.memo.len().saturating_add(1);
        if observed > self.arena.profile.max_comparison_pairs {
            return Err(self.arena.resource(
                ShortGameResource::ComparisonPairs,
                self.arena.profile.max_comparison_pairs,
                observed,
            ));
        }

        let left_options = self.arena.nodes[left].left.clone();
        let right_options = self.arena.nodes[right].right.clone();
        for left_option in left_options {
            if self.less_or_equal(right, left_option)? {
                self.memo.insert((left, right), false);
                return Ok(false);
            }
        }
        for right_option in right_options {
            if self.less_or_equal(right_option, left)? {
                self.memo.insert((left, right), false);
                return Ok(false);
            }
        }
        self.memo.insert((left, right), true);
        Ok(true)
    }
}

#[derive(Debug, Clone)]
enum Rewrite {
    DominatedLeft { option: NodeId },
    DominatedRight { option: NodeId },
    ReversibleLeft { option: NodeId, response: NodeId },
    ReversibleRight { option: NodeId, response: NodeId },
}

struct CanonicalEngine<'a> {
    comparison: ComparisonEngine<'a>,
    canonical_cache: HashMap<NodeId, NodeId>,
    rewrite_steps: usize,
}

impl CanonicalEngine<'_> {
    fn sorted_options(&mut self, options: &[NodeId]) -> Result<Vec<NodeId>, ShortGameError> {
        let mut keyed = options
            .iter()
            .map(|option| Ok((self.comparison.arena.serialization(*option)?, *option)))
            .collect::<Result<Vec<_>, ShortGameError>>()?;
        keyed.sort_by(|left, right| left.0.cmp(&right.0));
        keyed.dedup_by(|left, right| left.1 == right.1);
        Ok(keyed.into_iter().map(|(_, option)| option).collect())
    }

    fn find_rewrite(&mut self, current: NodeId) -> Result<Option<Rewrite>, ShortGameError> {
        let node = self.comparison.arena.nodes[current].clone();
        let left = self.sorted_options(&node.left)?;
        let right = self.sorted_options(&node.right)?;
        let mut candidates = Vec::<(String, Rewrite)>::new();

        for option in &left {
            let option_serialization = self.comparison.arena.serialization(*option)?;
            for witness in &left {
                if option == witness {
                    continue;
                }
                if self.comparison.less_or_equal(*option, *witness)? {
                    let witness_serialization = self.comparison.arena.serialization(*witness)?;
                    candidates.push((
                        format!("dominated_left\0{option_serialization}\0{witness_serialization}"),
                        Rewrite::DominatedLeft { option: *option },
                    ));
                }
            }
        }

        for option in &right {
            let option_serialization = self.comparison.arena.serialization(*option)?;
            for witness in &right {
                if option == witness {
                    continue;
                }
                if self.comparison.less_or_equal(*witness, *option)? {
                    let witness_serialization = self.comparison.arena.serialization(*witness)?;
                    candidates.push((
                        format!("dominated_right\0{option_serialization}\0{witness_serialization}"),
                        Rewrite::DominatedRight { option: *option },
                    ));
                }
            }
        }

        for option in &left {
            let option_serialization = self.comparison.arena.serialization(*option)?;
            let responses =
                self.sorted_options(&self.comparison.arena.nodes[*option].right.clone())?;
            for response in responses {
                if self.comparison.less_or_equal(response, current)? {
                    let response_serialization = self.comparison.arena.serialization(response)?;
                    candidates.push((
                        format!(
                            "reversible_left\0{option_serialization}\0{response_serialization}"
                        ),
                        Rewrite::ReversibleLeft {
                            option: *option,
                            response,
                        },
                    ));
                }
            }
        }

        for option in &right {
            let option_serialization = self.comparison.arena.serialization(*option)?;
            let responses =
                self.sorted_options(&self.comparison.arena.nodes[*option].left.clone())?;
            for response in responses {
                if self.comparison.less_or_equal(current, response)? {
                    let response_serialization = self.comparison.arena.serialization(response)?;
                    candidates.push((
                        format!(
                            "reversible_right\0{option_serialization}\0{response_serialization}"
                        ),
                        Rewrite::ReversibleRight {
                            option: *option,
                            response,
                        },
                    ));
                }
            }
        }

        candidates.sort_by(|left, right| left.0.cmp(&right.0));
        Ok(candidates.into_iter().next().map(|(_, rewrite)| rewrite))
    }

    fn apply_rewrite(
        &mut self,
        current: NodeId,
        rewrite: Rewrite,
    ) -> Result<NodeId, ShortGameError> {
        let observed = self.rewrite_steps.saturating_add(1);
        if observed > self.comparison.arena.profile.max_rewrite_steps {
            return Err(self.comparison.arena.resource(
                ShortGameResource::RewriteSteps,
                self.comparison.arena.profile.max_rewrite_steps,
                observed,
            ));
        }
        self.rewrite_steps = observed;

        let node = self.comparison.arena.nodes[current].clone();
        let mut left = node.left;
        let mut right = node.right;
        match rewrite {
            Rewrite::DominatedLeft { option } => left.retain(|candidate| *candidate != option),
            Rewrite::DominatedRight { option } => right.retain(|candidate| *candidate != option),
            Rewrite::ReversibleLeft { option, response } => {
                left.retain(|candidate| *candidate != option);
                left.extend(self.comparison.arena.nodes[response].left.iter().copied());
            }
            Rewrite::ReversibleRight { option, response } => {
                right.retain(|candidate| *candidate != option);
                right.extend(self.comparison.arena.nodes[response].right.iter().copied());
            }
        }
        self.comparison.arena.intern(left, right)
    }

    fn canonicalize(&mut self, root: NodeId) -> Result<NodeId, ShortGameError> {
        if let Some(canonical) = self.canonical_cache.get(&root) {
            return Ok(*canonical);
        }
        let source = self.comparison.arena.nodes[root].clone();
        let left = source
            .left
            .iter()
            .map(|option| self.canonicalize(*option))
            .collect::<Result<Vec<_>, _>>()?;
        let right = source
            .right
            .iter()
            .map(|option| self.canonicalize(*option))
            .collect::<Result<Vec<_>, _>>()?;
        let mut current = self.comparison.arena.intern(left, right)?;

        loop {
            let Some(rewrite) = self.find_rewrite(current)? else {
                break;
            };
            let next = self.apply_rewrite(current, rewrite)?;
            if next == current {
                return Err(ShortGameError::SemanticVerificationFailed);
            }
            current = next;
        }
        self.canonical_cache.insert(root, current);
        Ok(current)
    }
}

/// Computes the exact four-way normal-play relation under `profile`.
pub fn compare_short_game_bounded(
    left: &CGTValue,
    right: &CGTValue,
    profile: &ShortGameProfile,
) -> Result<ShortGameComparison, ShortGameError> {
    preflight_birthday(left, profile)?;
    preflight_birthday(right, profile)?;
    let mut engine = ComparisonEngine {
        arena: Arena::new(profile),
        memo: HashMap::new(),
    };
    let left_root = engine.arena.expand(left)?;
    engine.arena.enforce_source_node_limit(left_root)?;
    let right_root = engine.arena.expand(right)?;
    engine.arena.enforce_source_node_limit(right_root)?;
    let left_le_right = engine.less_or_equal(left_root, right_root)?;
    let right_le_left = engine.less_or_equal(right_root, left_root)?;
    Ok(match (left_le_right, right_le_left) {
        (true, true) => ShortGameComparison::Equal,
        (true, false) => ShortGameComparison::Less,
        (false, true) => ShortGameComparison::Greater,
        (false, false) => ShortGameComparison::Fuzzy,
    })
}

/// Tests exact normal-play equality by mutual bounded comparison.
pub fn equal_short_game_bounded(
    left: &CGTValue,
    right: &CGTValue,
    profile: &ShortGameProfile,
) -> Result<bool, ShortGameError> {
    compare_short_game_bounded(left, right, profile)
        .map(|comparison| comparison == ShortGameComparison::Equal)
}

/// Returns the domain-separated version-one identifier of canonical bytes.
#[must_use]
pub fn semantic_canonical_id_v1(canonical_serialization: &str) -> String {
    let mut bytes =
        Vec::with_capacity(SEMANTIC_CANONICAL_V1_PREFIX.len() + canonical_serialization.len());
    bytes.extend_from_slice(SEMANTIC_CANONICAL_V1_PREFIX);
    bytes.extend_from_slice(canonical_serialization.as_bytes());
    sha256_hex(&bytes)
}

/// Computes and audits the bounded semantic canonical form of `value`.
pub fn semantic_canonical_form_bounded(
    value: &CGTValue,
    profile: &ShortGameProfile,
) -> Result<SemanticCanonicalForm, ShortGameError> {
    preflight_birthday(value, profile)?;
    let mut engine = CanonicalEngine {
        comparison: ComparisonEngine {
            arena: Arena::new(profile),
            memo: HashMap::new(),
        },
        canonical_cache: HashMap::new(),
        rewrite_steps: 0,
    };
    let source_root = engine.comparison.arena.expand(value)?;
    engine
        .comparison
        .arena
        .enforce_source_node_limit(source_root)?;
    let canonical_root = engine.canonicalize(source_root)?;
    let canonical_birthday = engine.comparison.arena.nodes[canonical_root].birthday;
    if canonical_birthday > profile.max_canonical_birthday {
        return Err(limit_error(
            ShortGameResource::CanonicalBirthday,
            profile.max_canonical_birthday,
            u64::from(canonical_birthday),
        ));
    }

    if !engine
        .comparison
        .less_or_equal(source_root, canonical_root)?
        || !engine
            .comparison
            .less_or_equal(canonical_root, source_root)?
    {
        return Err(ShortGameError::SemanticVerificationFailed);
    }
    if engine.find_rewrite(canonical_root)?.is_some() {
        return Err(ShortGameError::SemanticVerificationFailed);
    }

    let canonical_serialization = engine.comparison.arena.serialization(canonical_root)?;
    let idempotent_root = engine.canonicalize(canonical_root)?;
    let idempotent_serialization = engine.comparison.arena.serialization(idempotent_root)?;
    if canonical_serialization != idempotent_serialization {
        return Err(ShortGameError::SemanticVerificationFailed);
    }

    let value_id = semantic_canonical_id_v1(&canonical_serialization);
    let canonical_game = engine.comparison.arena.explicit_value(canonical_root)?;
    Ok(SemanticCanonicalForm {
        canonical_game,
        canonical_serialization,
        value_id,
        canonical_birthday,
        rewrite_steps: engine.rewrite_steps,
        comparison_pairs: engine.comparison.memo.len(),
        intermediate_nodes: engine.comparison.arena.nodes.len(),
    })
}

fn insert_catalogue_form(
    rows: &mut BTreeMap<String, SemanticTargetCatalogueRow>,
    form: SemanticCanonicalForm,
) -> Result<(), ShortGameError> {
    let row = SemanticTargetCatalogueRow {
        canonical_game: form.canonical_game,
        canonical_serialization: form.canonical_serialization,
        value_id: form.value_id.clone(),
        birthday: form.canonical_birthday,
    };
    if let Some(existing) = rows.get(&form.value_id) {
        if existing.canonical_serialization != row.canonical_serialization {
            return Err(ShortGameError::SemanticVerificationFailed);
        }
        return Ok(());
    }
    rows.insert(form.value_id, row);
    Ok(())
}

fn subset_options(values: &[SemanticTargetCatalogueRow]) -> Vec<Vec<CGTValue>> {
    (0_usize..(1_usize << values.len()))
        .map(|mask| {
            values
                .iter()
                .enumerate()
                .filter(|(index, _)| mask & (1_usize << index) != 0)
                .map(|(_, row)| row.canonical_game.clone())
                .collect()
        })
        .collect()
}

fn generate_next_day_from_all_subsets(
    values: &[SemanticTargetCatalogueRow],
    profile: &ShortGameProfile,
) -> Result<BTreeMap<String, SemanticTargetCatalogueRow>, ShortGameError> {
    let subsets = subset_options(values);
    let mut rows = BTreeMap::new();
    for left in &subsets {
        for right in &subsets {
            let candidate = CGTValue::GameTree {
                left: left.clone(),
                right: right.clone(),
            };
            insert_catalogue_form(
                &mut rows,
                semantic_canonical_form_bounded(&candidate, profile)?,
            )?;
        }
    }
    Ok(rows)
}

fn enumerate_antichains(comparable: &[Vec<bool>]) -> Vec<Vec<usize>> {
    fn visit(
        index: usize,
        comparable: &[Vec<bool>],
        selected: &mut Vec<usize>,
        antichains: &mut Vec<Vec<usize>>,
    ) {
        if index == comparable.len() {
            antichains.push(selected.clone());
            return;
        }

        visit(index + 1, comparable, selected, antichains);
        if selected
            .iter()
            .all(|selected_index| !comparable[index][*selected_index])
        {
            selected.push(index);
            visit(index + 1, comparable, selected, antichains);
            selected.pop();
        }
    }

    let mut antichains = Vec::new();
    visit(0, comparable, &mut Vec::new(), &mut antichains);
    antichains.sort();
    antichains
}

/// Generates the complete semantic target catalogue born by birthday three.
///
/// The construction enumerates the four day-one literal games, the 256 day-two
/// literal games, and all ordered pairs of antichains in the 22-value
/// cumulative day-two order. Birthday four is outside this operation.
pub fn semantic_target_catalogue_birthday3_bounded(
    profile: &ShortGameProfile,
) -> Result<SemanticTargetCatalogue, ShortGameError> {
    let zero = semantic_canonical_form_bounded(&CGTValue::Integer(0), profile)?;
    let mut day0 = BTreeMap::new();
    insert_catalogue_form(&mut day0, zero)?;
    let day0_rows = day0.values().cloned().collect::<Vec<_>>();

    let day1 = generate_next_day_from_all_subsets(&day0_rows, profile)?;
    let day1_rows = day1.values().cloned().collect::<Vec<_>>();
    let day2 = generate_next_day_from_all_subsets(&day1_rows, profile)?;
    let day2_rows = day2.values().cloned().collect::<Vec<_>>();

    let mut comparable = vec![vec![false; day2_rows.len()]; day2_rows.len()];
    for left in 0..day2_rows.len() {
        for right in left + 1..day2_rows.len() {
            let relation = compare_short_game_bounded(
                &day2_rows[left].canonical_game,
                &day2_rows[right].canonical_game,
                profile,
            )?;
            let is_comparable = relation != ShortGameComparison::Fuzzy;
            comparable[left][right] = is_comparable;
            comparable[right][left] = is_comparable;
        }
    }
    let antichains = enumerate_antichains(&comparable);

    let mut day3 = BTreeMap::new();
    for left_indices in &antichains {
        let left = left_indices
            .iter()
            .map(|index| day2_rows[*index].canonical_game.clone())
            .collect::<Vec<_>>();
        for right_indices in &antichains {
            let right = right_indices
                .iter()
                .map(|index| day2_rows[*index].canonical_game.clone())
                .collect::<Vec<_>>();
            let candidate = CGTValue::GameTree {
                left: left.clone(),
                right,
            };
            insert_catalogue_form(
                &mut day3,
                semantic_canonical_form_bounded(&candidate, profile)?,
            )?;
        }
    }

    let rows = day3.into_values().collect::<Vec<_>>();
    let mut exact_birthday_counts = [0_usize; 4];
    for row in &rows {
        let Some(count) = exact_birthday_counts.get_mut(row.birthday as usize) else {
            return Err(ShortGameError::SemanticVerificationFailed);
        };
        *count += 1;
    }
    let cumulative_counts = [
        day0_rows.len(),
        day1_rows.len(),
        day2_rows.len(),
        rows.len(),
    ];
    let day3_candidate_pairs = antichains.len().saturating_mul(antichains.len());
    if cumulative_counts != [1, 4, 22, 1_474]
        || exact_birthday_counts != [1, 3, 18, 1_452]
        || antichains.len() != 98
        || day3_candidate_pairs != 9_604
    {
        return Err(ShortGameError::SemanticVerificationFailed);
    }

    Ok(SemanticTargetCatalogue {
        profile_id: profile.profile_id,
        maximum_birthday: 3,
        rows,
        cumulative_counts,
        exact_birthday_counts,
        day2_antichain_count: antichains.len(),
        day3_candidate_pairs,
    })
}

//! Thermograph: Combinatorial Game Theory & Surreal Numbers in Rust
//!
//! Provides thermograph utilities and stable canonical structural identities
//! for game trees, dyadic numbers, and infinitesimal values like *, ^, and v.
//!
//! Canonical identity in this crate normalizes represented structure only. It
//! is suitable for stable labels and digests, but it is not a proof of full CGT
//! equivalence between arbitrary game trees.

#[derive(Debug, Clone, PartialEq)]
pub enum CGTValue {
    Integer(i32),
    Dyadic(i32, u32), // e.g., 1/2, 3/4
    Star,             // Nimber *
    Up,               // ^
    Down,             // v
    GameTree {
        left: Vec<CGTValue>,
        right: Vec<CGTValue>,
    },
}

/// Stable value-class labels for `partizan.dataset_label.v0` exact-value payloads.
///
/// `Number` is the only class with a supported exact dyadic numeric value. For
/// `Star`, `Up`, `Down`, `Switch`, and `GameTree`, the public exact contract is
/// limited to canonical structural identity: value class, serialization, and
/// digest.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExactValueClass {
    Number,
    Star,
    Up,
    Down,
    Switch,
    GameTree,
}

impl ExactValueClass {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            ExactValueClass::Number => "number",
            ExactValueClass::Star => "star",
            ExactValueClass::Up => "up",
            ExactValueClass::Down => "down",
            ExactValueClass::Switch => "switch",
            ExactValueClass::GameTree => "game_tree",
        }
    }
}

/// Public exact-value payload for dataset labels.
///
/// The fields map directly to `partizan.dataset_label.v0` `exact.value` data:
/// a stable class, canonical serialization, digest of that serialization, and
/// exact dyadic data when the represented value is currently supported as an
/// exact number.
///
/// This payload intentionally does not claim full CGT equivalence for arbitrary
/// game trees. For non-numeric classes, `dyadic` is `None` even when an
/// approximate thermograph mean is available through the existing f32 APIs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExactValuePayload {
    pub value_class: ExactValueClass,
    pub canonical_serialization: String,
    pub digest: String,
    pub dyadic: Option<DyadicRational>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DyadicRational {
    numerator: i32,
    denominator_power: u32,
}

impl DyadicRational {
    #[must_use]
    pub fn new(numerator: i32, denominator_power: u32) -> Self {
        Self::try_new_i128(i128::from(numerator), denominator_power)
            .expect("i32 numerator should fit after dyadic normalization")
    }

    #[must_use]
    fn try_new_i128(mut numerator: i128, mut denominator_power: u32) -> Option<Self> {
        if numerator == 0 {
            return Some(Self {
                numerator: 0,
                denominator_power: 0,
            });
        }

        while denominator_power > 0 && numerator % 2 == 0 {
            numerator /= 2;
            denominator_power -= 1;
        }

        Some(Self {
            numerator: i32::try_from(numerator).ok()?,
            denominator_power,
        })
    }

    #[must_use]
    pub fn numerator(&self) -> i32 {
        self.numerator
    }

    #[must_use]
    pub fn denominator_power(&self) -> u32 {
        self.denominator_power
    }

    #[must_use]
    pub fn is_zero(&self) -> bool {
        self.numerator == 0
    }

    #[must_use]
    pub fn checked_negate(&self) -> Option<Self> {
        Self::try_new_i128(-i128::from(self.numerator), self.denominator_power)
    }

    #[must_use]
    pub fn negate(&self) -> Self {
        self.checked_negate()
            .expect("dyadic negation should fit in i32 numerator")
    }

    #[must_use]
    pub fn checked_add(&self, other: &Self) -> Option<Self> {
        let denominator_power = self.denominator_power.max(other.denominator_power);
        let left = checked_scale_dyadic_numerator(*self, denominator_power)?;
        let right = checked_scale_dyadic_numerator(*other, denominator_power)?;
        Self::try_new_i128(left.checked_add(right)?, denominator_power)
    }

    #[must_use]
    pub fn add(&self, other: &Self) -> Self {
        self.checked_add(other)
            .expect("dyadic addition should fit in i32 numerator")
    }

    #[must_use]
    pub fn checked_sub(&self, other: &Self) -> Option<Self> {
        self.checked_add(&other.checked_negate()?)
    }

    #[must_use]
    pub fn sub(&self, other: &Self) -> Self {
        self.checked_sub(other)
            .expect("dyadic subtraction should fit in i32 numerator")
    }

    #[must_use]
    pub fn to_cgt_value(&self) -> CGTValue {
        if self.denominator_power == 0 {
            CGTValue::Integer(self.numerator)
        } else {
            CGTValue::Dyadic(self.numerator, self.denominator_power)
        }
    }

    #[must_use]
    pub fn to_f32(&self) -> f32 {
        let den_i32 = i32::try_from(self.denominator_power).unwrap_or(i32::MAX);
        self.numerator as f32 / 2.0_f32.powi(den_i32)
    }

    #[must_use]
    pub fn canonical_serialization(&self) -> String {
        format!("Number({}/2^{})", self.numerator, self.denominator_power)
    }
}

#[must_use]
fn checked_scale_dyadic_numerator(dyadic: DyadicRational, denominator_power: u32) -> Option<i128> {
    let shift = denominator_power.checked_sub(dyadic.denominator_power)?;
    i128::from(dyadic.numerator).checked_shl(shift)
}

const FNV64_OFFSET_BASIS: u64 = 0xcbf2_9ce4_8422_2325;
const FNV64_PRIME: u64 = 0x0000_0100_0000_01b3;
const CANONICAL_PAYLOAD_V1_PREFIX: &[u8] = b"thermograph.canonical_payload.v1\n";

const SHA256_INITIAL_STATE: [u32; 8] = [
    0x6a09_e667,
    0xbb67_ae85,
    0x3c6e_f372,
    0xa54f_f53a,
    0x510e_527f,
    0x9b05_688c,
    0x1f83_d9ab,
    0x5be0_cd19,
];

const SHA256_K: [u32; 64] = [
    0x428a_2f98,
    0x7137_4491,
    0xb5c0_fbcf,
    0xe9b5_dba5,
    0x3956_c25b,
    0x59f1_11f1,
    0x923f_82a4,
    0xab1c_5ed5,
    0xd807_aa98,
    0x1283_5b01,
    0x2431_85be,
    0x550c_7dc3,
    0x72be_5d74,
    0x80de_b1fe,
    0x9bdc_06a7,
    0xc19b_f174,
    0xe49b_69c1,
    0xefbe_4786,
    0x0fc1_9dc6,
    0x240c_a1cc,
    0x2de9_2c6f,
    0x4a74_84aa,
    0x5cb0_a9dc,
    0x76f9_88da,
    0x983e_5152,
    0xa831_c66d,
    0xb003_27c8,
    0xbf59_7fc7,
    0xc6e0_0bf3,
    0xd5a7_9147,
    0x06ca_6351,
    0x1429_2967,
    0x27b7_0a85,
    0x2e1b_2138,
    0x4d2c_6dfc,
    0x5338_0d13,
    0x650a_7354,
    0x766a_0abb,
    0x81c2_c92e,
    0x9272_2c85,
    0xa2bf_e8a1,
    0xa81a_664b,
    0xc24b_8b70,
    0xc76c_51a3,
    0xd192_e819,
    0xd699_0624,
    0xf40e_3585,
    0x106a_a070,
    0x19a4_c116,
    0x1e37_6c08,
    0x2748_774c,
    0x34b0_bcb5,
    0x391c_0cb3,
    0x4ed8_aa4a,
    0x5b9c_ca4f,
    0x682e_6ff3,
    0x748f_82ee,
    0x78a5_636f,
    0x84c8_7814,
    0x8cc7_0208,
    0x90be_fffa,
    0xa450_6ceb,
    0xbef9_a3f7,
    0xc671_78f2,
];

#[must_use]
fn stable_hash_bytes(bytes: &[u8]) -> u64 {
    let mut hash = FNV64_OFFSET_BASIS;
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(FNV64_PRIME);
    }
    hash
}

#[must_use]
fn sha256_digest(bytes: &[u8]) -> [u8; 32] {
    let bit_len = u64::try_from(bytes.len())
        .expect("canonical payload length should fit in u64")
        .checked_mul(8)
        .expect("canonical payload bit length should fit in u64");
    let mut message = bytes.to_vec();
    message.push(0x80);
    while message.len() % 64 != 56 {
        message.push(0);
    }
    message.extend_from_slice(&bit_len.to_be_bytes());

    let mut state = SHA256_INITIAL_STATE;
    for chunk in message.chunks_exact(64) {
        let mut schedule = [0_u32; 64];
        for (i, word) in schedule.iter_mut().take(16).enumerate() {
            let offset = i * 4;
            *word = u32::from_be_bytes([
                chunk[offset],
                chunk[offset + 1],
                chunk[offset + 2],
                chunk[offset + 3],
            ]);
        }
        for i in 16..64 {
            let s0 = schedule[i - 15].rotate_right(7)
                ^ schedule[i - 15].rotate_right(18)
                ^ (schedule[i - 15] >> 3);
            let s1 = schedule[i - 2].rotate_right(17)
                ^ schedule[i - 2].rotate_right(19)
                ^ (schedule[i - 2] >> 10);
            schedule[i] = schedule[i - 16]
                .wrapping_add(s0)
                .wrapping_add(schedule[i - 7])
                .wrapping_add(s1);
        }

        let mut a = state[0];
        let mut b = state[1];
        let mut c = state[2];
        let mut d = state[3];
        let mut e = state[4];
        let mut f = state[5];
        let mut g = state[6];
        let mut h = state[7];

        for i in 0..64 {
            let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let ch = (e & f) ^ ((!e) & g);
            let temp1 = h
                .wrapping_add(s1)
                .wrapping_add(ch)
                .wrapping_add(SHA256_K[i])
                .wrapping_add(schedule[i]);
            let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let maj = (a & b) ^ (a & c) ^ (b & c);
            let temp2 = s0.wrapping_add(maj);

            h = g;
            g = f;
            f = e;
            e = d.wrapping_add(temp1);
            d = c;
            c = b;
            b = a;
            a = temp1.wrapping_add(temp2);
        }

        state[0] = state[0].wrapping_add(a);
        state[1] = state[1].wrapping_add(b);
        state[2] = state[2].wrapping_add(c);
        state[3] = state[3].wrapping_add(d);
        state[4] = state[4].wrapping_add(e);
        state[5] = state[5].wrapping_add(f);
        state[6] = state[6].wrapping_add(g);
        state[7] = state[7].wrapping_add(h);
    }

    let mut digest = [0_u8; 32];
    for (i, word) in state.iter().enumerate() {
        digest[i * 4..i * 4 + 4].copy_from_slice(&word.to_be_bytes());
    }
    digest
}

#[must_use]
fn hex_lower(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut output = Vec::with_capacity(bytes.len() * 2);
    for byte in bytes {
        output.push(HEX[usize::from(byte >> 4)]);
        output.push(HEX[usize::from(byte & 0x0f)]);
    }
    String::from_utf8(output).expect("hex bytes are valid UTF-8")
}

#[derive(Debug, Clone, PartialEq)]
pub struct PiecewiseLinear {
    pub points: Vec<(f32, f32)>,
    pub final_slope: f32,
}

impl PiecewiseLinear {
    #[must_use]
    pub fn eval(&self, t: f32) -> f32 {
        if self.points.is_empty() {
            return 0.0;
        }
        if t <= self.points[0].0 {
            return self.points[0].1;
        }
        for i in 0..self.points.len() - 1 {
            if t <= self.points[i + 1].0 {
                let dt = self.points[i + 1].0 - self.points[i].0;
                let dy = self.points[i + 1].1 - self.points[i].1;
                return self.points[i].1 + (t - self.points[i].0) * (dy / dt);
            }
        }
        let last = self.points.last().unwrap();
        last.1 + self.final_slope * (t - last.0)
    }

    #[must_use]
    pub fn minus_t(&self) -> PiecewiseLinear {
        let points = self.points.iter().map(|&(t, y)| (t, y - t)).collect();
        PiecewiseLinear {
            points,
            final_slope: self.final_slope - 1.0,
        }
    }

    #[must_use]
    pub fn plus_t(&self) -> PiecewiseLinear {
        let points = self.points.iter().map(|&(t, y)| (t, y + t)).collect();
        PiecewiseLinear {
            points,
            final_slope: self.final_slope + 1.0,
        }
    }

    #[must_use]
    pub fn combine(a: &PiecewiseLinear, b: &PiecewiseLinear, is_max: bool) -> PiecewiseLinear {
        let mut t_candidates = Vec::new();
        for &(t, _) in &a.points {
            t_candidates.push(t);
        }
        for &(t, _) in &b.points {
            t_candidates.push(t);
        }

        t_candidates.sort_by(|x, y| x.partial_cmp(y).unwrap());
        t_candidates.dedup();

        let mut all_t = Vec::new();
        for i in 0..t_candidates.len().saturating_sub(1) {
            let t1 = t_candidates[i];
            let t2 = t_candidates[i + 1];
            all_t.push(t1);

            let a1 = a.eval(t1);
            let a2 = a.eval(t2);
            let b1 = b.eval(t1);
            let b2 = b.eval(t2);

            if (a1 > b1 && a2 < b2) || (a1 < b1 && a2 > b2) {
                let slope_a = (a2 - a1) / (t2 - t1);
                let slope_b = (b2 - b1) / (t2 - t1);
                let t_int = t1 + (b1 - a1) / (slope_a - slope_b);
                all_t.push(t_int);
            }
        }
        if let Some(&last) = t_candidates.last() {
            all_t.push(last);
        }

        if let Some(&t_last) = all_t.last() {
            let a_last = a.eval(t_last);
            let b_last = b.eval(t_last);
            let slope_a = a.final_slope;
            let slope_b = b.final_slope;

            if (a_last > b_last && slope_a < slope_b) || (a_last < b_last && slope_a > slope_b) {
                let t_int = t_last + (b_last - a_last) / (slope_a - slope_b);
                if t_int > t_last {
                    all_t.push(t_int);
                }
            }
        }

        all_t.sort_by(|x, y| x.partial_cmp(y).unwrap());
        all_t.dedup();

        let mut points = Vec::new();
        for t in all_t {
            let y_a = a.eval(t);
            let y_b = b.eval(t);
            let y = if is_max { y_a.max(y_b) } else { y_a.min(y_b) };
            points.push((t, y));
        }

        let mut simplified: Vec<(f32, f32)> = Vec::new();
        for p in points {
            if simplified.len() < 2 {
                simplified.push(p);
            } else {
                let p1 = simplified[simplified.len() - 2];
                let p2 = simplified[simplified.len() - 1];
                if (p2.0 - p1.0).abs() < 1e-6 {
                    simplified.pop();
                    simplified.push(p);
                    continue;
                }
                let slope1 = (p2.1 - p1.1) / (p2.0 - p1.0);
                if (p.0 - p2.0).abs() < 1e-6 {
                    simplified.pop();
                    simplified.push(p);
                    continue;
                }
                let slope2 = (p.1 - p2.1) / (p.0 - p2.0);
                if (slope1 - slope2).abs() < 1e-4 {
                    simplified.pop();
                    simplified.push(p);
                } else {
                    simplified.push(p);
                }
            }
        }

        PiecewiseLinear {
            points: simplified,
            final_slope: if is_max {
                a.final_slope.max(b.final_slope)
            } else {
                a.final_slope.min(b.final_slope)
            },
        }
    }

    #[must_use]
    pub fn max(a: &PiecewiseLinear, b: &PiecewiseLinear) -> PiecewiseLinear {
        Self::combine(a, b, true)
    }

    #[must_use]
    pub fn min(a: &PiecewiseLinear, b: &PiecewiseLinear) -> PiecewiseLinear {
        Self::combine(a, b, false)
    }

    #[must_use]
    pub fn intersect(left: &PiecewiseLinear, right: &PiecewiseLinear) -> (f32, f32) {
        let mut all_t = Vec::new();
        for &(t, _) in &left.points {
            all_t.push(t);
        }
        for &(t, _) in &right.points {
            all_t.push(t);
        }
        all_t.sort_by(|x, y| x.partial_cmp(y).unwrap());
        all_t.dedup();

        if all_t.is_empty() {
            return (-1.0, 0.0);
        }

        let mut start_t = all_t[0];
        if start_t > -1.0 {
            start_t = -1.0;
        }

        if left.eval(start_t) <= right.eval(start_t) {
            return (
                start_t,
                f32::midpoint(left.eval(start_t), right.eval(start_t)),
            );
        }

        if start_t == -1.0 && all_t[0] > -1.0 {
            all_t.insert(0, -1.0);
        }

        for i in 0..all_t.len().saturating_sub(1) {
            let t1 = all_t[i];
            let t2 = all_t[i + 1];
            let l1 = left.eval(t1);
            let r1 = right.eval(t1);
            let l2 = left.eval(t2);
            let r2 = right.eval(t2);

            if l1 > r1 && l2 <= r2 {
                let slope_l = (l2 - l1) / (t2 - t1);
                let slope_r = (r2 - r1) / (t2 - t1);
                let t_int = t1 + (r1 - l1) / (slope_l - slope_r);
                return (t_int, left.eval(t_int));
            }
        }

        let t_last = *all_t.last().unwrap();
        let l_last = left.eval(t_last);
        let r_last = right.eval(t_last);
        if l_last > r_last {
            let slope_l = left.final_slope;
            let slope_r = right.final_slope;
            let t_int = t_last + (r_last - l_last) / (slope_l - slope_r);
            return (t_int, left.eval(t_int));
        }

        (-1.0, 0.0)
    }

    #[must_use]
    pub fn truncate(pwl: &PiecewiseLinear, t_g: f32, m_g: f32) -> PiecewiseLinear {
        let mut new_points = Vec::new();
        for &(t, y) in &pwl.points {
            if t < t_g - 1e-4 {
                new_points.push((t, y));
            }
        }
        new_points.push((t_g, m_g));
        PiecewiseLinear {
            points: new_points,
            final_slope: 0.0,
        }
    }
}

impl CGTValue {
    #[must_use]
    pub fn is_number(&self) -> bool {
        matches!(self, CGTValue::Integer(_) | CGTValue::Dyadic(_, _))
    }

    #[must_use]
    pub fn try_to_f32(&self) -> Option<f32> {
        self.try_to_dyadic().map(|dyadic| dyadic.to_f32())
    }

    #[must_use]
    pub fn to_f32(&self) -> f32 {
        self.try_to_f32()
            .expect("CGTValue::to_f32 requires Integer or Dyadic")
    }

    #[must_use]
    pub fn try_to_dyadic(&self) -> Option<DyadicRational> {
        match self {
            CGTValue::Integer(i) => Some(DyadicRational::new(*i, 0)),
            CGTValue::Dyadic(num, den) => Some(DyadicRational::new(*num, *den)),
            _ => None,
        }
    }

    #[must_use]
    pub fn value_class(&self) -> ExactValueClass {
        match self {
            CGTValue::Integer(_) | CGTValue::Dyadic(_, _) => ExactValueClass::Number,
            CGTValue::Star => ExactValueClass::Star,
            CGTValue::Up => ExactValueClass::Up,
            CGTValue::Down => ExactValueClass::Down,
            CGTValue::GameTree { left, right } if is_simple_switch(left, right) => {
                ExactValueClass::Switch
            }
            CGTValue::GameTree { .. } => ExactValueClass::GameTree,
        }
    }

    /// Returns stable exact-value fields suitable for
    /// `partizan.dataset_label.v0` `exact.value`.
    ///
    /// Integer and dyadic values include exact normalized dyadic data. Other
    /// value classes include canonical structural identity only; callers should
    /// not infer an exact numeric value from thermograph f32 outputs.
    #[must_use]
    pub fn exact_value_payload(&self) -> ExactValuePayload {
        ExactValuePayload {
            value_class: self.value_class(),
            canonical_serialization: self.canonical_serialization(),
            digest: self.stable_canonical_digest(),
            dyadic: self.try_to_dyadic(),
        }
    }

    #[must_use]
    pub fn canonical_serialization(&self) -> String {
        if let Some(dyadic) = self.try_to_dyadic() {
            return dyadic.canonical_serialization();
        }

        match self {
            CGTValue::Integer(_) | CGTValue::Dyadic(_, _) => unreachable!(),
            CGTValue::Star => "Star".to_string(),
            CGTValue::Up => "Up".to_string(),
            CGTValue::Down => "Down".to_string(),
            CGTValue::GameTree { left, right } => {
                let mut left_options = left
                    .iter()
                    .map(CGTValue::canonical_serialization)
                    .collect::<Vec<_>>();
                left_options.sort();
                left_options.dedup();

                let mut right_options = right
                    .iter()
                    .map(CGTValue::canonical_serialization)
                    .collect::<Vec<_>>();
                right_options.sort();
                right_options.dedup();

                format!(
                    "GameTree(L[{}];R[{}])",
                    left_options.join(","),
                    right_options.join(",")
                )
            }
        }
    }

    #[must_use]
    pub fn canonical_bytes(&self) -> Vec<u8> {
        self.canonical_serialization().into_bytes()
    }

    #[must_use]
    pub fn stable_canonical_hash(&self) -> u64 {
        stable_hash_bytes(&self.canonical_bytes())
    }

    #[must_use]
    pub fn stable_canonical_digest(&self) -> String {
        format!("{:016x}", self.stable_canonical_hash())
    }

    #[must_use]
    pub fn canonical_payload_v1_bytes(&self) -> Vec<u8> {
        let canonical_serialization = self.canonical_serialization();
        let mut bytes =
            Vec::with_capacity(CANONICAL_PAYLOAD_V1_PREFIX.len() + canonical_serialization.len());
        bytes.extend_from_slice(CANONICAL_PAYLOAD_V1_PREFIX);
        bytes.extend_from_slice(canonical_serialization.as_bytes());
        bytes
    }

    #[must_use]
    pub fn digest_v1_sha256(&self) -> String {
        hex_lower(&sha256_digest(&self.canonical_payload_v1_bytes()))
    }

    #[must_use]
    pub fn options(&self) -> (Vec<CGTValue>, Vec<CGTValue>) {
        match self {
            CGTValue::Integer(n) => {
                if *n > 0 {
                    (vec![CGTValue::Integer(n - 1)], vec![])
                } else if *n < 0 {
                    (vec![], vec![CGTValue::Integer(n + 1)])
                } else {
                    (vec![], vec![])
                }
            }
            CGTValue::Dyadic(num, den_pow) => {
                if *den_pow == 0 {
                    return CGTValue::Integer(*num).options();
                }
                let mut l_num = *num - 1;
                let mut l_den = *den_pow;
                while l_den > 0 && l_num % 2 == 0 {
                    l_num /= 2;
                    l_den -= 1;
                }
                let left = if l_den == 0 {
                    CGTValue::Integer(l_num)
                } else {
                    CGTValue::Dyadic(l_num, l_den)
                };

                let mut r_num = *num + 1;
                let mut r_den = *den_pow;
                while r_den > 0 && r_num % 2 == 0 {
                    r_num /= 2;
                    r_den -= 1;
                }
                let right = if r_den == 0 {
                    CGTValue::Integer(r_num)
                } else {
                    CGTValue::Dyadic(r_num, r_den)
                };

                (vec![left], vec![right])
            }
            CGTValue::Star => (vec![CGTValue::Integer(0)], vec![CGTValue::Integer(0)]),
            CGTValue::Up => (vec![CGTValue::Integer(0)], vec![CGTValue::Star]),
            CGTValue::Down => (vec![CGTValue::Star], vec![CGTValue::Integer(0)]),
            CGTValue::GameTree { left, right } => (left.clone(), right.clone()),
        }
    }

    #[must_use]
    pub fn negate(&self) -> Self {
        if let Some(dyadic) = self.try_to_dyadic()
            && let Some(negated) = dyadic.checked_negate()
        {
            return negated.to_cgt_value();
        }

        match self {
            CGTValue::Integer(_) | CGTValue::Dyadic(_, _) => {
                let (left, right) = self.options();
                CGTValue::GameTree {
                    left: right.iter().map(CGTValue::negate).collect(),
                    right: left.iter().map(CGTValue::negate).collect(),
                }
            }
            CGTValue::Star => CGTValue::Star,
            CGTValue::Up => CGTValue::Down,
            CGTValue::Down => CGTValue::Up,
            CGTValue::GameTree { left, right } => CGTValue::GameTree {
                left: right.iter().map(CGTValue::negate).collect(),
                right: left.iter().map(CGTValue::negate).collect(),
            },
        }
    }

    #[must_use]
    pub fn add(&self, other: &Self) -> Self {
        if let (Some(left), Some(right)) = (self.try_to_dyadic(), other.try_to_dyadic())
            && let Some(sum) = left.checked_add(&right)
        {
            return sum.to_cgt_value();
        }

        if self.try_to_dyadic().is_some_and(|dyadic| dyadic.is_zero()) {
            return other.clone();
        }
        if other.try_to_dyadic().is_some_and(|dyadic| dyadic.is_zero()) {
            return self.clone();
        }

        let (self_left, self_right) = self.options();
        let (other_left, other_right) = other.options();
        let mut left = Vec::with_capacity(self_left.len() + other_left.len());
        let mut right = Vec::with_capacity(self_right.len() + other_right.len());

        for option in self_left {
            left.push(option.add(other));
        }
        for option in other_left {
            left.push(self.add(&option));
        }
        for option in self_right {
            right.push(option.add(other));
        }
        for option in other_right {
            right.push(self.add(&option));
        }

        CGTValue::GameTree { left, right }
    }

    #[must_use]
    pub fn sub(&self, other: &Self) -> Self {
        self.add(&other.negate())
    }

    #[must_use]
    pub fn sum_all<I, V>(values: I) -> Self
    where
        I: IntoIterator<Item = V>,
        V: std::borrow::Borrow<CGTValue>,
    {
        values
            .into_iter()
            .fold(CGTValue::Integer(0), |sum, value| sum.add(value.borrow()))
    }

    #[must_use]
    pub fn ge(&self, other: &Self) -> bool {
        let (_, x_r) = self.options();
        let (y_l, _) = other.options();
        for xr in x_r {
            if other.ge(&xr) {
                return false;
            }
        }
        for yl in y_l {
            if yl.ge(self) {
                return false;
            }
        }
        true
    }

    #[must_use]
    pub fn le(&self, other: &Self) -> bool {
        other.ge(self)
    }

    #[must_use]
    pub fn exact_thermograph(
        &self,
    ) -> (f32, f32, Option<PiecewiseLinear>, Option<PiecewiseLinear>) {
        if self.is_number() {
            let m = self.to_f32();
            let pwl = PiecewiseLinear {
                points: vec![(-1.0, m)],
                final_slope: 0.0,
            };
            return (-1.0, m, Some(pwl.clone()), Some(pwl));
        }
        let (left, right) = self.options();
        if left.is_empty() && right.is_empty() {
            let pwl = PiecewiseLinear {
                points: vec![(-1.0, 0.0)],
                final_slope: 0.0,
            };
            return (-1.0, 0.0, Some(pwl.clone()), Some(pwl));
        }

        let mut left_scaffolds = Vec::new();
        for l in left {
            let (_, _, _, r_scaffold) = l.exact_thermograph();
            if let Some(r) = r_scaffold {
                left_scaffolds.push(r.minus_t());
            }
        }

        let mut right_scaffolds = Vec::new();
        for r in right {
            let (_, _, l_scaffold, _) = r.exact_thermograph();
            if let Some(l) = l_scaffold {
                right_scaffolds.push(l.plus_t());
            }
        }

        let l_scaffold = left_scaffolds
            .into_iter()
            .reduce(|a, b| PiecewiseLinear::max(&a, &b));
        let r_scaffold = right_scaffolds
            .into_iter()
            .reduce(|a, b| PiecewiseLinear::min(&a, &b));

        let (t_g, m_g) = match (l_scaffold.as_ref(), r_scaffold.as_ref()) {
            (None, None) => (-1.0, 0.0),
            (Some(l), None) => (-1.0, l.eval(-1.0)),
            (None, Some(r)) => (-1.0, r.eval(-1.0)),
            (Some(l), Some(r)) => PiecewiseLinear::intersect(l, r),
        };

        let final_l = Some(match l_scaffold {
            Some(l) => PiecewiseLinear::truncate(&l, t_g, m_g),
            None => PiecewiseLinear {
                points: vec![(t_g, m_g)],
                final_slope: 0.0,
            },
        });
        let final_r = Some(match r_scaffold {
            Some(r) => PiecewiseLinear::truncate(&r, t_g, m_g),
            None => PiecewiseLinear {
                points: vec![(t_g, m_g)],
                final_slope: 0.0,
            },
        });

        (t_g, m_g, final_l, final_r)
    }

    #[must_use]
    pub fn thermograph(&self) -> (f32, f32) {
        let (t_g, m_g, _, _) = self.exact_thermograph();
        (t_g, m_g)
    }

    #[must_use]
    pub fn temperature(&self) -> f32 {
        self.thermograph().0
    }

    #[must_use]
    pub fn mean_value(&self) -> f32 {
        self.thermograph().1
    }

    #[must_use]
    pub fn left_scaffold(&self, t: f32) -> f32 {
        if self.is_number() {
            return self.to_f32();
        }
        let (_, _, l_scaffold, _) = self.exact_thermograph();
        if let Some(l) = l_scaffold {
            l.eval(t)
        } else {
            f32::NEG_INFINITY
        }
    }

    #[must_use]
    pub fn right_scaffold(&self, t: f32) -> f32 {
        if self.is_number() {
            return self.to_f32();
        }
        let (_, _, _, r_scaffold) = self.exact_thermograph();
        if let Some(r) = r_scaffold {
            r.eval(t)
        } else {
            f32::INFINITY
        }
    }

    #[must_use]
    pub fn simplify(&self) -> Self {
        let CGTValue::GameTree { left, right } = self else {
            return self.clone();
        };

        let l_simp: Vec<CGTValue> = left.iter().map(CGTValue::simplify).collect();
        let r_simp: Vec<CGTValue> = right.iter().map(CGTValue::simplify).collect();

        let mut current_game = CGTValue::GameTree {
            left: l_simp,
            right: r_simp,
        };

        let mut changed = true;
        while changed {
            changed = false;
            let mut rev_index_l = None;
            let mut replacement_l = Vec::new();

            if let CGTValue::GameTree { left, right: _ } = &current_game {
                for (i, l) in left.iter().enumerate() {
                    let mut reversible_by = None;
                    let (_, l_r) = l.options();
                    for r_opt in l_r {
                        if r_opt.le(&current_game) {
                            reversible_by = Some(r_opt);
                            break;
                        }
                    }
                    if let Some(rev) = reversible_by {
                        rev_index_l = Some(i);
                        let (rev_l, _) = rev.options();
                        for rl in rev_l {
                            replacement_l.push(rl.simplify());
                        }
                        break;
                    }
                }
            }
            if let Some(idx) = rev_index_l {
                if let CGTValue::GameTree { left, .. } = &mut current_game {
                    left.remove(idx);
                    for rl in replacement_l.into_iter().rev() {
                        left.insert(idx, rl);
                    }
                }
                changed = true;
                continue;
            }

            let mut rev_index_r = None;
            let mut replacement_r = Vec::new();

            if let CGTValue::GameTree { left: _, right } = &current_game {
                for (i, r) in right.iter().enumerate() {
                    let mut reversible_by = None;
                    let (r_l, _) = r.options();
                    for l_opt in r_l {
                        if l_opt.ge(&current_game) {
                            reversible_by = Some(l_opt);
                            break;
                        }
                    }
                    if let Some(rev) = reversible_by {
                        rev_index_r = Some(i);
                        let (_, rev_r) = rev.options();
                        for rr in rev_r {
                            replacement_r.push(rr.simplify());
                        }
                        break;
                    }
                }
            }
            if let Some(idx) = rev_index_r {
                if let CGTValue::GameTree { right, .. } = &mut current_game {
                    right.remove(idx);
                    for rr in replacement_r.into_iter().rev() {
                        right.insert(idx, rr);
                    }
                }
                changed = true;
                continue;
            }
        }

        let (mut final_l, mut final_r) = match current_game {
            CGTValue::GameTree { left, right } => (left, right),
            _ => unreachable!(),
        };

        let mut i = 0;
        while i < final_l.len() {
            let mut dominated = false;
            for j in 0..final_l.len() {
                if i == j {
                    continue;
                }
                if final_l[j].ge(&final_l[i]) && (!final_l[i].ge(&final_l[j]) || j < i) {
                    dominated = true;
                    break;
                }
            }
            if dominated {
                final_l.remove(i);
            } else {
                i += 1;
            }
        }

        let mut i = 0;
        while i < final_r.len() {
            let mut dominated = false;
            for j in 0..final_r.len() {
                if i == j {
                    continue;
                }
                if final_r[j].le(&final_r[i]) && (!final_r[i].le(&final_r[j]) || j < i) {
                    dominated = true;
                    break;
                }
            }
            if dominated {
                final_r.remove(i);
            } else {
                i += 1;
            }
        }

        CGTValue::GameTree {
            left: final_l,
            right: final_r,
        }
    }
}

fn is_simple_switch(left: &[CGTValue], right: &[CGTValue]) -> bool {
    if left.len() != 1 || right.len() != 1 {
        return false;
    }

    let Some(left_value) = left[0].try_to_dyadic() else {
        return false;
    };
    let Some(right_value) = right[0].try_to_dyadic() else {
        return false;
    };

    dyadic_greater_than(left_value, right_value)
}

fn dyadic_greater_than(left: DyadicRational, right: DyadicRational) -> bool {
    let scale = left.denominator_power.max(right.denominator_power);
    if scale <= 120 {
        let left_scaled = i128::from(left.numerator) << (scale - left.denominator_power);
        let right_scaled = i128::from(right.numerator) << (scale - right.denominator_power);
        return left_scaled > right_scaled;
    }

    left.to_f32() > right.to_f32()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sha256_helper_matches_standard_vector() {
        assert_eq!(
            hex_lower(&sha256_digest(b"abc")),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad",
        );
    }

    #[test]
    fn test_simplify_one_minus_one() {
        let g = CGTValue::GameTree {
            left: vec![CGTValue::Integer(1)],
            right: vec![CGTValue::Integer(-1)],
        };
        let simplified = g.simplify();
        let t = simplified.temperature();
        let m = simplified.mean_value();
        assert!((t - 1.0).abs() < 1e-3);
        assert!((m - 0.0).abs() < 1e-3);
    }

    #[test]
    fn test_domination() {
        let g = CGTValue::GameTree {
            left: vec![CGTValue::Integer(1), CGTValue::Integer(0)],
            right: vec![CGTValue::Integer(-1), CGTValue::Integer(0)],
        };
        let simplified = g.simplify();
        let t = simplified.temperature();
        let m = simplified.mean_value();
        assert!((t - 1.0).abs() < 1e-3);
        assert!((m - 0.0).abs() < 1e-3);
    }

    #[test]
    fn test_large_dyadic_conversion_does_not_shift_panic() {
        let tiny = CGTValue::Dyadic(1, 64);

        assert!(tiny.to_f32() > 0.0);
    }

    #[test]
    #[should_panic(expected = "CGTValue::to_f32 requires Integer or Dyadic")]
    fn test_non_numeric_to_f32_is_explicit_error() {
        let _ = CGTValue::Star.to_f32();
    }
}

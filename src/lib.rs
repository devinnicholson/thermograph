//! Thermograph: Combinatorial Game Theory & Surreal Numbers in Rust
//!
//! Provides mathematically rigorous canonical forms for game trees,
//! surreal numbers, and infinitesimal values like *, ^, and v.

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
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PiecewiseLinear {
    pub points: Vec<(f32, f32)>,
    pub final_slope: f32,
}

impl PiecewiseLinear {
    #[must_use] 
    pub fn eval(&self, t: f32) -> f32 {
        if self.points.is_empty() { return 0.0; }
        if t <= self.points[0].0 {
            return self.points[0].1;
        }
        for i in 0..self.points.len() - 1 {
            if t <= self.points[i+1].0 {
                let dt = self.points[i+1].0 - self.points[i].0;
                let dy = self.points[i+1].1 - self.points[i].1;
                return self.points[i].1 + (t - self.points[i].0) * (dy / dt);
            }
        }
        let last = self.points.last().unwrap();
        last.1 + self.final_slope * (t - last.0)
    }

    #[must_use] 
    pub fn minus_t(&self) -> PiecewiseLinear {
        let points = self.points.iter().map(|&(t, y)| (t, y - t)).collect();
        PiecewiseLinear { points, final_slope: self.final_slope - 1.0 }
    }
    
    #[must_use] 
    pub fn plus_t(&self) -> PiecewiseLinear {
        let points = self.points.iter().map(|&(t, y)| (t, y + t)).collect();
        PiecewiseLinear { points, final_slope: self.final_slope + 1.0 }
    }

    #[must_use] 
    pub fn combine(a: &PiecewiseLinear, b: &PiecewiseLinear, is_max: bool) -> PiecewiseLinear {
        let mut t_candidates = Vec::new();
        for &(t, _) in &a.points { t_candidates.push(t); }
        for &(t, _) in &b.points { t_candidates.push(t); }
        
        t_candidates.sort_by(|x, y| x.partial_cmp(y).unwrap());
        t_candidates.dedup();
        
        let mut all_t = Vec::new();
        for i in 0..t_candidates.len().saturating_sub(1) {
            let t1 = t_candidates[i];
            let t2 = t_candidates[i+1];
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
            final_slope: if is_max { a.final_slope.max(b.final_slope) } else { a.final_slope.min(b.final_slope) },
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
        for &(t, _) in &left.points { all_t.push(t); }
        for &(t, _) in &right.points { all_t.push(t); }
        all_t.sort_by(|x, y| x.partial_cmp(y).unwrap());
        all_t.dedup();
        
        if all_t.is_empty() { return (-1.0, 0.0); }
        
        let mut start_t = all_t[0];
        if start_t > -1.0 { start_t = -1.0; }
        
        if left.eval(start_t) <= right.eval(start_t) {
            return (start_t, f32::midpoint(left.eval(start_t), right.eval(start_t)));
        }
        
        if start_t == -1.0 && all_t[0] > -1.0 {
            all_t.insert(0, -1.0);
        }
        
        for i in 0..all_t.len().saturating_sub(1) {
            let t1 = all_t[i];
            let t2 = all_t[i+1];
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
        PiecewiseLinear { points: new_points, final_slope: 0.0 }
    }
}

impl CGTValue {
    #[must_use] 
    pub fn is_number(&self) -> bool {
        match self {
            CGTValue::Integer(_) | CGTValue::Dyadic(_, _) => true,
            _ => false,
        }
    }

    #[must_use] 
    pub fn to_f32(&self) -> f32 {
        match self {
            CGTValue::Integer(i) => *i as f32,
            CGTValue::Dyadic(num, den) => *num as f32 / (1_u32 << *den) as f32,
            _ => 0.0,
        }
    }

    #[must_use] 
    pub fn options(&self) -> (Vec<CGTValue>, Vec<CGTValue>) {
        match self {
            CGTValue::Integer(n) => {
                if *n > 0 { (vec![CGTValue::Integer(n - 1)], vec![]) } 
                else if *n < 0 { (vec![], vec![CGTValue::Integer(n + 1)]) } 
                else { (vec![], vec![]) }
            }
            CGTValue::Dyadic(num, den_pow) => {
                if *den_pow == 0 { return CGTValue::Integer(*num).options(); }
                let mut l_num = *num - 1; let mut l_den = *den_pow;
                while l_den > 0 && l_num % 2 == 0 { l_num /= 2; l_den -= 1; }
                let left = if l_den == 0 { CGTValue::Integer(l_num) } else { CGTValue::Dyadic(l_num, l_den) };

                let mut r_num = *num + 1; let mut r_den = *den_pow;
                while r_den > 0 && r_num % 2 == 0 { r_num /= 2; r_den -= 1; }
                let right = if r_den == 0 { CGTValue::Integer(r_num) } else { CGTValue::Dyadic(r_num, r_den) };
                
                (vec![left], vec![right])
            }
            CGTValue::Star => (vec![CGTValue::Integer(0)], vec![CGTValue::Integer(0)]),
            CGTValue::Up => (vec![CGTValue::Integer(0)], vec![CGTValue::Star]),
            CGTValue::Down => (vec![CGTValue::Star], vec![CGTValue::Integer(0)]),
            CGTValue::GameTree { left, right } => (left.clone(), right.clone()),
        }
    }

    #[must_use] 
    pub fn ge(&self, other: &Self) -> bool {
        let (_, x_r) = self.options();
        let (y_l, _) = other.options();
        for xr in x_r { if other.ge(&xr) { return false; } }
        for yl in y_l { if yl.ge(self) { return false; } }
        true
    }

    #[must_use] 
    pub fn le(&self, other: &Self) -> bool {
        other.ge(self)
    }

    #[must_use] 
    pub fn exact_thermograph(&self) -> (f32, f32, Option<PiecewiseLinear>, Option<PiecewiseLinear>) {
        if self.is_number() {
            let m = self.to_f32();
            let pwl = PiecewiseLinear { points: vec![(-1.0, m)], final_slope: 0.0 };
            return (-1.0, m, Some(pwl.clone()), Some(pwl));
        }
        let (left, right) = self.options();
        if left.is_empty() && right.is_empty() {
            let pwl = PiecewiseLinear { points: vec![(-1.0, 0.0)], final_slope: 0.0 };
            return (-1.0, 0.0, Some(pwl.clone()), Some(pwl));
        }
        
        let mut left_scaffolds = Vec::new();
        for l in left {
            let (_, _, _, r_scaffold) = l.exact_thermograph();
            if let Some(r) = r_scaffold { left_scaffolds.push(r.minus_t()); }
        }
        
        let mut right_scaffolds = Vec::new();
        for r in right {
            let (_, _, l_scaffold, _) = r.exact_thermograph();
            if let Some(l) = l_scaffold { right_scaffolds.push(l.plus_t()); }
        }
        
        let l_scaffold = left_scaffolds.into_iter().reduce(|a, b| PiecewiseLinear::max(&a, &b));
        let r_scaffold = right_scaffolds.into_iter().reduce(|a, b| PiecewiseLinear::min(&a, &b));
        
        let (t_g, m_g) = match (l_scaffold.as_ref(), r_scaffold.as_ref()) {
            (None, None) => (-1.0, 0.0),
            (Some(l), None) => (-1.0, l.eval(-1.0)),
            (None, Some(r)) => (-1.0, r.eval(-1.0)),
            (Some(l), Some(r)) => PiecewiseLinear::intersect(l, r),
        };
        
        let final_l = Some(match l_scaffold {
            Some(l) => PiecewiseLinear::truncate(&l, t_g, m_g),
            None => PiecewiseLinear { points: vec![(t_g, m_g)], final_slope: 0.0 },
        });
        let final_r = Some(match r_scaffold {
            Some(r) => PiecewiseLinear::truncate(&r, t_g, m_g),
            None => PiecewiseLinear { points: vec![(t_g, m_g)], final_slope: 0.0 },
        });
        
        (t_g, m_g, final_l, final_r)
    }

    #[must_use] 
    pub fn thermograph(&self) -> (f32, f32) {
        let (t_g, m_g, _, _) = self.exact_thermograph();
        (t_g, m_g)
    }

    #[must_use] 
    pub fn temperature(&self) -> f32 { self.thermograph().0 }
    
    #[must_use] 
    pub fn mean_value(&self) -> f32 { self.thermograph().1 }

    #[must_use] 
    pub fn left_scaffold(&self, t: f32) -> f32 {
        if self.is_number() { return self.to_f32(); }
        let (_, _, l_scaffold, _) = self.exact_thermograph();
        if let Some(l) = l_scaffold { l.eval(t) } else { f32::NEG_INFINITY }
    }

    #[must_use] 
    pub fn right_scaffold(&self, t: f32) -> f32 {
        if self.is_number() { return self.to_f32(); }
        let (_, _, _, r_scaffold) = self.exact_thermograph();
        if let Some(r) = r_scaffold { r.eval(t) } else { f32::INFINITY }
    }
    
    #[must_use] 
    pub fn simplify(&self) -> Self {
        let (left, right) = self.options();
        let l_simp: Vec<CGTValue> = left.iter().map(CGTValue::simplify).collect();
        let r_simp: Vec<CGTValue> = right.iter().map(CGTValue::simplify).collect();
        
        let mut current_game = CGTValue::GameTree { left: l_simp, right: r_simp };
        
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
                if i == j { continue; }
                if final_l[j].ge(&final_l[i]) && (!final_l[i].ge(&final_l[j]) || j < i) {
                    dominated = true;
                    break;
                }
            }
            if dominated { final_l.remove(i); } else { i += 1; }
        }

        let mut i = 0;
        while i < final_r.len() {
            let mut dominated = false;
            for j in 0..final_r.len() {
                if i == j { continue; }
                if final_r[j].le(&final_r[i]) && (!final_r[i].le(&final_r[j]) || j < i) {
                    dominated = true;
                    break;
                }
            }
            if dominated { final_r.remove(i); } else { i += 1; }
        }
        
        CGTValue::GameTree { left: final_l, right: final_r }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}

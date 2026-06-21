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

impl CGTValue {
    pub fn is_number(&self) -> bool {
        match self {
            CGTValue::Integer(_) | CGTValue::Dyadic(_, _) => true,
            _ => false,
        }
    }

    pub fn to_f32(&self) -> f32 {
        match self {
            CGTValue::Integer(i) => *i as f32,
            CGTValue::Dyadic(num, den) => *num as f32 / (1_u32 << *den) as f32,
            _ => 0.0,
        }
    }

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
                let left = if l_den == 0 { CGTValue::Integer(l_num) } else { CGTValue::Dyadic(l_num, l_den) };

                let mut r_num = *num + 1;
                let mut r_den = *den_pow;
                while r_den > 0 && r_num % 2 == 0 {
                    r_num /= 2;
                    r_den -= 1;
                }
                let right = if r_den == 0 { CGTValue::Integer(r_num) } else { CGTValue::Dyadic(r_num, r_den) };
                
                (vec![left], vec![right])
            }
            CGTValue::Star => (vec![CGTValue::Integer(0)], vec![CGTValue::Integer(0)]),
            CGTValue::Up => (vec![CGTValue::Integer(0)], vec![CGTValue::Star]),
            CGTValue::Down => (vec![CGTValue::Star], vec![CGTValue::Integer(0)]),
            CGTValue::GameTree { left, right } => (left.clone(), right.clone()),
        }
    }

    pub fn ge(&self, other: &Self) -> bool {
        let (_, x_r) = self.options();
        let (y_l, _) = other.options();
        for xr in x_r {
            if other.ge(&xr) { return false; }
        }
        for yl in y_l {
            if yl.ge(self) { return false; }
        }
        true
    }

    pub fn le(&self, other: &Self) -> bool {
        other.ge(self)
    }

    pub fn temperature(&self) -> f32 {
        self.thermograph().0
    }
    
    pub fn mean_value(&self) -> f32 {
        self.thermograph().1
    }

    pub fn thermograph(&self) -> (f32, f32) {
        if self.is_number() {
            return (-1.0, self.to_f32());
        }
        let (left, right) = self.options();
        if left.is_empty() && right.is_empty() {
            return (-1.0, 0.0);
        }
        
        let mut low = -1.0;
        let mut high = 100.0;
        for _ in 0..50 {
            let mid = (low + high) / 2.0;
            let f_t = self.eval_f(mid);
            let g_t = self.eval_g(mid);
            if f_t <= g_t {
                high = mid;
            } else {
                low = mid;
            }
        }
        let t = (low + high) / 2.0;
        let m = (self.eval_f(t) + self.eval_g(t)) / 2.0;
        (t, m)
    }

    fn eval_f(&self, t: f32) -> f32 {
        let (left, _) = self.options();
        if left.is_empty() { return f32::NEG_INFINITY; }
        left.iter().map(|a| a.right_scaffold(t) - t).fold(f32::NEG_INFINITY, f32::max)
    }

    fn eval_g(&self, t: f32) -> f32 {
        let (_, right) = self.options();
        if right.is_empty() { return f32::INFINITY; }
        right.iter().map(|b| b.left_scaffold(t) + t).fold(f32::INFINITY, f32::min)
    }

    pub fn left_scaffold(&self, t: f32) -> f32 {
        if self.is_number() { return self.to_f32(); }
        let (t_g, m_g) = self.thermograph();
        if t >= t_g { return m_g; }
        self.eval_f(t)
    }

    pub fn right_scaffold(&self, t: f32) -> f32 {
        if self.is_number() { return self.to_f32(); }
        let (t_g, m_g) = self.thermograph();
        if t >= t_g { return m_g; }
        self.eval_g(t)
    }
    
    pub fn simplify(&self) -> Self {
        let (left, right) = self.options();
        let mut l_simp: Vec<CGTValue> = left.iter().map(|o| o.simplify()).collect();
        let mut r_simp: Vec<CGTValue> = right.iter().map(|o| o.simplify()).collect();
        
        let mut changed = true;
        while changed {
            changed = false;
            let current_game = CGTValue::GameTree { left: l_simp.clone(), right: r_simp.clone() };

            let mut new_l = Vec::new();
            for l in &l_simp {
                let mut reversible_by = None;
                let (_, l_r) = l.options();
                for r_opt in l_r {
                    if r_opt.le(&current_game) {
                        reversible_by = Some(r_opt);
                        break;
                    }
                }
                if let Some(rev) = reversible_by {
                    let (rev_l, _) = rev.options();
                    for rl in rev_l {
                        new_l.push(rl.simplify());
                    }
                    changed = true;
                } else {
                    new_l.push(l.clone());
                }
            }
            l_simp = new_l;

            let mut new_r = Vec::new();
            for r in &r_simp {
                let mut reversible_by = None;
                let (r_l, _) = r.options();
                for l_opt in r_l {
                    if l_opt.ge(&current_game) {
                        reversible_by = Some(l_opt);
                        break;
                    }
                }
                if let Some(rev) = reversible_by {
                    let (_, rev_r) = rev.options();
                    for rr in rev_r {
                        new_r.push(rr.simplify());
                    }
                    changed = true;
                } else {
                    new_r.push(r.clone());
                }
            }
            r_simp = new_r;
        }

        let mut final_l = Vec::new();
        for i in 0..l_simp.len() {
            let mut dominated = false;
            for j in 0..l_simp.len() {
                if i == j { continue; }
                if l_simp[j].ge(&l_simp[i]) && (!l_simp[i].ge(&l_simp[j]) || j < i) {
                    dominated = true;
                    break;
                }
            }
            if !dominated { final_l.push(l_simp[i].clone()); }
        }

        let mut final_r = Vec::new();
        for i in 0..r_simp.len() {
            let mut dominated = false;
            for j in 0..r_simp.len() {
                if i == j { continue; }
                if r_simp[j].le(&r_simp[i]) && (!r_simp[i].le(&r_simp[j]) || j < i) {
                    dominated = true;
                    break;
                }
            }
            if !dominated { final_r.push(r_simp[i].clone()); }
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
        println!("t: {}, m: {}", t, m);
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

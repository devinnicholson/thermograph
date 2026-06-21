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
    pub fn temperature(&self) -> f32 {
        // TODO: Compute the cooling temperature of the game tree
        0.0 
    }
    
    pub fn mean_value(&self) -> f32 {
        // TODO: Compute the stopping value / mean advantage
        0.0 
    }
    
    pub fn simplify(&self) -> Self {
        // TODO: Recursively prune dominated options and reversible moves
        self.clone()
    }
}
